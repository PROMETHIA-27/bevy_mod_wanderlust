use crate::controller::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;

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
    #[reflect(ignore)]
    pub cast_collider: Collider,
    /// Set of entities that should be ignored when ground casting.
    pub exclude_from_ground: HashSet<Entity>,
    /// The maximum angle that the ground can be, in radians, before it is no longer considered suitable for being "grounded" on.
    ///
    /// For example, if this is set to `π/4` (45 degrees), then a player standing on a slope steeper than 45 degrees will slip and fall, and will not have
    /// their jump refreshed by landing on that surface.
    ///
    /// This is done by ignoring the ground during ground casting.
    pub max_ground_angle: f32,
}

impl Default for GroundCaster {
    fn default() -> Self {
        Self {
            skip_ground_check_timer: 0.0,
            skip_ground_check_override: false,
            cast_origin: Vec3::ZERO,
            cast_length: 1.0,
            cast_collider: Collider::ball(0.45),
            exclude_from_ground: default(),
            max_ground_angle: 75.0 * (std::f32::consts::PI / 180.0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ground {
    pub entity: Entity,
    pub cast: CastResult,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
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

    ctx: Res<RapierContext>,

    mut ground_shape_casts: Local<Vec<(Entity, Toi)>>,
    mut ground_ray_casts: Local<Vec<(Entity, RayIntersection)>>,
) {
    let dt = ctx.integration_parameters.dt;
    for (entity, tf, gravity, mut caster, mut cast) in &mut casters {
        let casted = if caster.skip_ground_check_timer == 0.0 && !caster.skip_ground_check_override
        {
            let shape_desc = ShapeDesc {
                shape_pos: tf.transform_point(caster.cast_origin),
                shape_rot: tf.to_scale_rotation_translation().1,
                shape_vel: -gravity.up_vector,
                shape: &caster.cast_collider,
            };

            intersections_with_shape_cast(
                &ctx,
                &shape_desc,
                caster.cast_length,
                QueryFilter::new().exclude_sensors().predicate(&|collider| {
                    collider != entity && !caster.exclude_from_ground.contains(&collider)
                }),
                &mut ground_shape_casts,
            );

            if let Some((entity, toi)) = ground_shape_casts
                .iter()
                .find(|(_, i)| {
                    i.status != TOIStatus::Penetrating
                        && i.normal1.angle_between(gravity.up_vector) <= caster.max_ground_angle
                })
                .cloned()
            {
                Some((
                    entity,
                    CastResult {
                        toi: toi.toi,
                        witness: toi.witness1,
                        normal: toi.normal1,
                    },
                ))
            } else {
                intersections_with_ray_cast(
                    &ctx,
                    &shape_desc,
                    caster.cast_length,
                    QueryFilter::new().exclude_sensors().predicate(&|collider| {
                        collider != entity && !caster.exclude_from_ground.contains(&collider)
                    }),
                    &mut ground_ray_casts,
                );

                if let Some((entity, inter)) = ground_ray_casts
                    .iter()
                    .find(|(_, i)| {
                        i.normal.angle_between(gravity.up_vector) <= caster.max_ground_angle
                    })
                    .cloned()
                {
                    Some((
                        entity,
                        CastResult {
                            toi: inter.toi,
                            witness: inter.point,
                            normal: inter.normal,
                        },
                    ))
                } else {
                    None
                }
            }
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

                let ground_velocity = velocities.get(ground_entity).copied().unwrap_or(Velocity::default());

                let global = globals.get(ground_entity).unwrap_or(&GlobalTransform::IDENTITY);
                let com = global.transform_point(local_com);
                let velocity = ground_velocity.linvel + ground_velocity.angvel.cross(result.witness - com);

                *cast = GroundCast::Touching(Ground {
                    entity: ground_entity,
                    cast: result,
                    linear_velocity: velocity,
                    angular_velocity: ground_velocity.angvel,
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
        let float_offset = if let GroundCast::Touching(ground) = cast {
            Some(ground.cast.toi - float.distance)
        } else {
            None
        };

        grounded.0 = float_offset
            .map(|offset| offset <= float.max_offset && offset >= float.min_offset)
            .unwrap_or(false);
    }
}

struct ShapeDesc<'c> {
    shape_pos: Vec3,
    shape_rot: Quat,
    shape_vel: Vec3,
    shape: &'c Collider,
}

#[derive(Default, Debug, Copy, Clone, Reflect)]
pub struct CastResult {
    pub toi: f32,
    pub normal: Vec3,
    pub witness: Vec3,
}

fn intersections_with_shape_cast(
    ctx: &RapierContext,
    shape: &ShapeDesc,
    mut max_toi: f32,
    filter: QueryFilter,
    collisions: &mut Vec<(Entity, Toi)>,
) {
    collisions.clear();

    let ShapeDesc {
        shape_pos,
        shape_rot,
        shape_vel,
        shape,
    } = *shape;
    let offset = 0.1;
    let shape_pos = shape_pos - shape_vel * offset;
    max_toi += offset;

    let orig_predicate = filter.predicate;

    loop {
        let predicate = |entity| {
            !collisions.iter().any(|coll| coll.0 == entity)
                && orig_predicate.map(|pred| pred(entity)).unwrap_or(true)
        };
        let filter = filter.predicate(&predicate);

        if let Some((entity, mut toi)) =
            ctx.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter)
        {
            toi.toi -= offset;
            collisions.push((entity, toi));
        } else {
            break;
        }
    }
}

fn intersections_with_ray_cast(
    ctx: &RapierContext,
    shape: &ShapeDesc,
    max_toi: f32,
    filter: QueryFilter,
    collisions: &mut Vec<(Entity, RayIntersection)>,
) {
    collisions.clear();

    let orig_predicate = filter.predicate;

    let ShapeDesc {
        shape_pos,
        shape_vel,
        shape,
        ..
    } = *shape;

    // We need to offset it so the point of contact is identical to the shape cast.
    let offset = shape
        .cast_local_ray(Vec3::ZERO, shape_vel, 10.0, false)
        .unwrap_or(0.);
    let shape_pos = shape_pos + shape_vel * offset;

    loop {
        let predicate = |entity| {
            !collisions.iter().any(|coll| coll.0 == entity)
                && orig_predicate.map(|pred| pred(entity)).unwrap_or(true)
        };
        let filter = filter.predicate(&predicate);

        if let Some((entity, inter)) =
            ctx.cast_ray_and_get_normal(shape_pos, shape_vel, max_toi, true, filter)
        {
            collisions.push((entity, inter));
        } else {
            break;
        }
    }
}
