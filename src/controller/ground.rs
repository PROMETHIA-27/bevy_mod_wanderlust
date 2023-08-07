use crate::controller::*;
use bevy::utils::HashSet;
use bevy_rapier3d::{na::Isometry3, prelude::*};

/// How to detect if something below the controller is suitable
/// for standing on.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct GroundCaster {
    /// A timer to track how long to skip the ground check for (see [`jump_skip_ground_check_duration`](ControllerSettings::jump_skip_ground_check_duration)).
    pub skip_ground_check_timer: f32,
    /// Override skip ground check. If true, never checks for the ground.
    pub skip_ground_check_override: bool,
    /// An offset to start the ground check from, relative to the character's origin.
    pub cast_origin: Vec3,
    /// How long of a ray to cast to detect the ground. Setting this unnecessarily high will permanently count the player as grounded,
    /// and too low will allow the player to slip and become disconnected from the ground easily.
    pub cast_length: f32,
    /// What shape of ray to cast. See [`Collider`] and [`RapierContext::cast_shape`](bevy_rapier::prelude::RapierContext).
    ///
    /// The default is the controller's base collider.
    #[reflect(ignore)]
    pub cast_collider: Option<Collider>,
    /// Set of entities that should be ignored when ground casting.
    pub exclude_from_ground: HashSet<Entity>,

    /// Threshold, in radians, of when a controller will start to slip on a surface.
    ///
    /// The controller will still be able to jump and overall be considered grounded.
    pub unstable_ground_angle: f32,
    /// The maximum angle that the ground can be, in radians, before it is no longer considered suitable for being "grounded" on.
    ///
    /// For example, if this is set to `π/4` (45 degrees), then a controller standing on a slope steeper than 45 degrees will slip and fall, and will not have
    /// their jump refreshed by landing on that surface.
    pub max_ground_angle: f32,
}

impl Default for GroundCaster {
    fn default() -> Self {
        Self {
            skip_ground_check_timer: 0.0,
            skip_ground_check_override: false,
            cast_origin: Vec3::ZERO,
            cast_length: 1.0,
            cast_collider: None,
            exclude_from_ground: default(),
            unstable_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
            max_ground_angle: 70.0 * (std::f32::consts::PI / 180.0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ground {
    /// Entity found in ground cast.
    pub entity: Entity,
    /// Specifics of the ground contact.
    pub cast: CastResult,
    /// Is this ground stable for the collider.
    pub stable: bool,
    /// Is this ground viable for the collider.
    pub viable: bool,
    /// Velocity at the point of contact.
    pub point_velocity: Velocity,
}

/// The cached ground cast. Contains the entity hit, the hit info, and velocity of the entity
/// hit.
#[derive(Component, Default)]
pub enum GroundCast {
    Touching(Ground),
    Last(Ground),
    #[default]
    None,
}

impl GroundCast {
    pub fn last(&self) -> Option<&Ground> {
        match self {
            Self::Touching(ground) | Self::Last(ground) => Some(ground),
            Self::None => None,
        }
    }

    pub fn grounded(&self) -> bool {
        match self {
            Self::Touching(_) => true,
            Self::Last(_) | Self::None => false,
        }
    }

    pub fn into_last(&mut self) {
        match self {
            GroundCast::Touching(ground) => {
                *self = GroundCast::Last(ground.clone());
            }
            _ => {}
        }
    }
}

/// Is the character grounded?
#[derive(Component, Default, Reflect, Deref)]
#[reflect(Component, Default)]
pub struct Grounded(pub bool);

/// Force applied to the ground the controller is on.
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GroundForce {
    /// Change in linear velocity.
    pub linear: Vec3,
    /// Change in angular velocity.
    pub angular: Vec3,
}

/// Performs groundcasting and updates controller state accordingly.
pub fn find_ground(
    mut casters: Query<(
        Entity,
        &GlobalTransform,
        &Gravity,
        &mut GroundCaster,
        &mut GroundCast,
    )>,

    velocities: Query<&Velocity>,
    masses: Query<&ReadMassProperties>,
    globals: Query<&GlobalTransform>,
    colliders: Query<&Collider>,

    ctx: Res<RapierContext>,

    mut ground_shape_casts: Local<Vec<(Entity, Toi)>>,
    mut ground_ray_casts: Local<Vec<(Entity, RayIntersection)>>,
) {
    let dt = ctx.integration_parameters.dt;
    for (entity, tf, gravity, mut caster, mut cast) in &mut casters {
        let casted = if caster.skip_ground_check_timer == 0.0 && !caster.skip_ground_check_override
        {
            let cast_position = tf.transform_point(caster.cast_origin);
            let cast_rotation = tf.to_scale_rotation_translation().1;
            let cast_direction = -gravity.up_vector;
            let Ok(caster_collider) = colliders.get(entity) else { continue };
            let shape = caster.cast_collider.as_ref().unwrap_or(caster_collider);

            let predicate =
                |collider| collider != entity && !caster.exclude_from_ground.contains(&collider);
            let filter = QueryFilter::new().exclude_sensors().predicate(&predicate);

            ground_cast(
                &*ctx,
                caster.max_ground_angle,
                &colliders,
                &globals,
                cast_position,
                cast_rotation,
                cast_direction,
                &shape,
                caster.cast_length,
                filter,
            )
        } else {
            caster.skip_ground_check_timer = (caster.skip_ground_check_timer - dt).max(0.0);
            None
        };

        match casted {
            Some((entity, result)) => {
                let ground_entity = ctx.collider_parent(entity).unwrap_or(entity);

                let mass = if let Ok(mass) = masses.get(ground_entity) {
                    mass.0.clone()
                } else {
                    MassProperties::default()
                };

                let local_com = mass.local_center_of_mass;

                let ground_velocity = velocities
                    .get(ground_entity)
                    .copied()
                    .unwrap_or(Velocity::default());

                let global = globals
                    .get(ground_entity)
                    .unwrap_or(&GlobalTransform::IDENTITY);
                let com = global.transform_point(local_com);
                let velocity =
                    ground_velocity.linvel + ground_velocity.angvel.cross(result.witness - com);

                let (stable, viable) = if result.normal.length() > 0.0 {
                    let ground_angle = result.normal.angle_between(gravity.up_vector);
                    info!("ground_angle: {:?}", ground_angle * 180.0 / std::f32::consts::PI);
                    let viable = ground_angle <= caster.max_ground_angle;
                    let stable = ground_angle <= caster.unstable_ground_angle && viable;
                    (stable, viable)
                } else {
                    (false, false)
                };

                *cast = GroundCast::Touching(Ground {
                    entity: ground_entity,
                    cast: result,
                    stable: stable,
                    viable: viable,
                    point_velocity: Velocity {
                        linvel: velocity,
                        angvel: ground_velocity.angvel,
                    },
                });
            }
            None => {
                cast.into_last();
            }
        };

        // If we hit something, just get back up instead of waiting.
        if ctx.contacts_with(entity).next().is_some() {
            caster.skip_ground_check_timer = 0.0;
        }
    }
}

pub fn determine_groundedness(mut query: Query<(&Float, &GroundCast, &mut Grounded)>) {
    for (float, cast, mut grounded) in &mut query {
        if let GroundCast::Touching(ground) = cast {
            let offset = ground.cast.toi - float.distance;
            grounded.0 = offset <= float.max_offset && offset >= float.min_offset;
        } else {
            grounded.0 = false;
        };
    }
}

#[derive(Default, Debug, Copy, Clone, Reflect)]
pub struct CastResult {
    pub toi: f32,
    pub normal: Vec3,
    pub witness: Vec3,
}

impl From<Toi> for CastResult {
    fn from(toi: Toi) -> Self {
        Self {
            toi: toi.toi,
            normal: toi.normal1,
            witness: toi.witness1,
        }
    }
}

impl From<RayIntersection> for CastResult {
    fn from(intersection: RayIntersection) -> Self {
        Self {
            toi: intersection.toi,
            normal: intersection.normal,
            witness: intersection.point,
        }
    }
}

/// Robust casting to find the ground beneath the controller.
///
/// This has fallbacks to make sure we catch non-convex colliders.
pub fn ground_cast(
    ctx: &RapierContext,
    max_angle: f32,
    colliders: &Query<&Collider>,
    globals: &Query<&GlobalTransform>,
    mut shape_pos: Vec3,
    shape_rot: Quat,
    shape_vel: Vec3,
    shape: &Collider,
    max_toi: f32,
    filter: QueryFilter,
) -> Option<(Entity, CastResult)> {
    let raycast_filter = filter.clone();
    let mut shapecast_filter = filter.clone();
    for _ in 0..12 {
        if let Some((entity, toi)) =
            ctx.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter)
        {
            if toi.status != TOIStatus::Penetrating {
                let ground_angle = toi.normal1.angle_between(shape_vel);
                if ground_angle >= max_angle {
                    if let Some(old_predicate) = shapecast_filter.predicate.take() {
                        let entity = entity.clone();
                        let new_predicate =
                            |collider| collider != entity && (old_predicate)(collider);
                        shapecast_filter = shapecast_filter.predicate(&new_predicate);
                    }

                    continue;
                }
                return Some((entity, toi.into()));
            }

            match (globals.get(entity), colliders.get(entity)) {
                (Ok(ground_global), Ok(ground_collider)) => {
                    let cast_iso = Isometry3 {
                        translation: shape_pos.into(),
                        rotation: shape_rot.into(),
                    };

                    let (_, ground_rotation, ground_translation) =
                        ground_global.to_scale_rotation_translation();
                    let ground_iso = Isometry3 {
                        translation: ground_translation.into(),
                        rotation: ground_rotation.into(),
                    };

                    if let Ok(Some(contact)) = bevy_rapier3d::parry::query::contact(
                        &cast_iso,
                        &*shape.raw,
                        &ground_iso,
                        &*ground_collider.raw,
                        0.0,
                    ) {
                        let normal: Vec3 = contact.normal2.into();
                        // This prevents some issues where we get a near 0.0 time-of-impact due to floating point imprecision.
                        const EXTRA_CORRECTION: f32 = 1.5;
                        let correction = normal * (-contact.dist).max(0.05) * EXTRA_CORRECTION;
                        shape_pos += correction;
                    }
                }
                _ => {}
            };
        } else {
            return None;
        }
    }

    // We need to offset it so the point of contact is identical to the shape cast.
    let offset = shape
        .cast_local_ray(Vec3::ZERO, shape_vel, 10.0, false)
        .unwrap_or(0.);
    shape_pos = shape_pos + shape_vel * offset;

    //ctx.cast_ray_and_get_normal(shape_pos, shape_vel, max_toi, true, filter)
    //.map(|(entity, inter)| (entity, inter.into()))
    None
}

/*
fn intersections_with_ray_cast(
    ctx: &RapierContext,
    max_toi: f32,
    filter: QueryFilter,
) {
    let offset = shape
        .cast_local_ray(Vec3::ZERO, shape_vel, 10.0, false)
        .unwrap_or(0.);
    let shape_pos = shape_pos + shape_vel * offset;

    loop {
        if let Some((entity, inter)) =
            ctx.cast_ray_and_get_normal(shape_pos, shape_vel, max_toi, true, filter)
        {
            collisions.push((entity, inter));
        } else {
            break;
        }
    }
}
*/
