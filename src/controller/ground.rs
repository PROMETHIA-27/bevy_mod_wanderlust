use crate::controller::*;
use bevy::utils::HashSet;
use bevy_rapier3d::{
    na::Isometry3,
    parry::{
        bounding_volume::BoundingVolume,
        query::{DefaultQueryDispatcher, PersistentQueryDispatcher},
    },
    rapier::{self, geometry::ContactManifold},
};

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
            cast_length: 1.5,
            cast_collider: None,
            exclude_from_ground: default(),
            unstable_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
            max_ground_angle: 60.0 * (std::f32::consts::PI / 180.0),
        }
    }
}

/// Information about the ground entity/where we are touching it.
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
    /// Angular velocity of the ground body.
    pub angular_velocity: Vec3,
    /// Linear velocity of the ground body.
    pub linear_velocity: Vec3,
    /// Linear velocity at the point of contact.
    pub point_velocity: Vec3,
}

/// The cached ground cast. Contains the entity hit, the hit info, and velocity of the entity
/// hit.
#[derive(Component, Default)]
pub struct GroundCast {
    /// Ground that was found this frame,
    /// this might not be viable for standing on.
    pub current: Option<Ground>,
    /// Cached *stable* ground that was found in the past.
    pub viable: ViableGround,
}

/// Current/last viable ground.
#[derive(Default)]
pub enum ViableGround {
    /// This will stay the viable ground until we leave the ground entirely.
    Ground(Ground),
    /// Cached viable ground.
    Last(Ground),
    /// No stable ground.
    #[default]
    None,
}

impl ViableGround {
    /// Update the viable ground depending on the current ground cast.
    pub fn update(&mut self, ground: Option<Ground>) {
        match ground {
            Some(ground) => {
                if ground.viable {
                    *self = ViableGround::Ground(ground);
                }
            }
            None => {
                self.into_last();
            }
        }
    }

    /// Archive this ground cast.
    pub fn into_last(&mut self) {
        match self {
            ViableGround::Ground(ground) => {
                *self = ViableGround::Last(ground.clone());
            }
            _ => {}
        }
    }

    /// Last ground we touched, this includes the ground we are currently touching.
    pub fn last(&self) -> Option<&Ground> {
        match self {
            Self::Ground(ground) | Self::Last(ground) => Some(ground),
            Self::None => None,
        }
    }
}

impl GroundCast {
    /// Given new information on a ground cast, update what we know.
    pub fn update(&mut self, ground: Option<Ground>) {
        self.current = ground.clone();
        self.viable.update(ground);
    }

    /// Are we currently touching the ground?
    pub fn grounded(&self) -> bool {
        match self.viable {
            ViableGround::Ground(_) => true,
            _ => false,
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
    time: Res<Time>,
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
    mut gizmos: Gizmos,
) {
    let dt = ctx.integration_parameters.dt;
    if time.delta_seconds() == 0.0 {
        return;
    }

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
                &colliders,
                &globals,
                cast_position,
                cast_rotation,
                cast_direction,
                &shape,
                caster.cast_length,
                filter,
                &mut gizmos,
            )
        } else {
            caster.skip_ground_check_timer = (caster.skip_ground_check_timer - dt).max(0.0);
            None
        };

        let next_ground = match casted {
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
                let point_velocity =
                    ground_velocity.linvel + ground_velocity.angvel.cross(result.point - com);

                let (stable, viable) = if result.normal.length() > 0.0 {
                    let ground_angle = result.normal.angle_between(gravity.up_vector);
                    let viable = ground_angle <= caster.max_ground_angle;
                    let stable = ground_angle <= caster.unstable_ground_angle && viable;
                    (stable, viable)
                } else {
                    (false, false)
                };

                Some(Ground {
                    entity: ground_entity,
                    cast: result,
                    stable: stable,
                    viable: viable,
                    linear_velocity: ground_velocity.linvel,
                    angular_velocity: ground_velocity.angvel,
                    point_velocity: point_velocity,
                })
            }
            None => None,
        };

        cast.update(next_ground);

        // If we hit something, just get back up instead of waiting.
        if ctx.contacts_with(entity).next().is_some() {
            caster.skip_ground_check_timer = 0.0;
        }
    }
}

/// Are we currently touching the ground with a fudge factor included.
pub fn determine_groundedness(
    mut query: Query<(
        &GlobalTransform,
        &Gravity,
        &Float,
        &GroundCast,
        &ControllerVelocity,
        &mut Grounded,
    )>,
) {
    for (global, gravity, float, cast, velocity, mut grounded) in &mut query {
        grounded.0 = false;
        if let Some(ground) = cast.current {
            if ground.viable {
                let up_velocity = velocity.linear.dot(gravity.up_vector);
                let translation = global.translation();
                let updated_toi =
                    translation.dot(gravity.up_vector) - ground.cast.point.dot(gravity.up_vector);
                //gizmos.sphere(ground.cast.point, Quat::IDENTITY, 0.3, Color::RED);
                //gizmos.sphere(translation, Quat::IDENTITY, 0.3, Color::GREEN);
                let offset = float.distance - updated_toi;

                //let up_velocity = up_velocity.clamp(-float.distance, float.distance);
                // Loosen constraints based on velocity.
                let max = if up_velocity > float.max_offset {
                    float.max_offset + up_velocity
                } else {
                    float.max_offset
                };
                let min = if up_velocity < float.min_offset {
                    float.min_offset + up_velocity
                } else {
                    float.min_offset
                };
                grounded.0 = offset >= min && offset <= max;
                /*
                info!(
                    "grounded: {:?}, {:.3?} <= {:.3?} <= {:.3?}",
                    grounded.0, min, offset, max
                );
                */
            }
        };
    }
}

/// Details about a shape/ray-cast.
#[derive(Default, Debug, Copy, Clone, Reflect)]
pub struct CastResult {
    /// Time-of-impact to the other shape.
    pub toi: f32,
    /// Normal of the other shape.
    pub normal: Vec3,
    /// Witness point for the shape/ray cast.
    pub point: Vec3,
}

impl CastResult {
    /// Use the first shape in the shape-cast as the cast result.
    pub fn from_toi1(toi: Toi) -> Self {
        Self {
            toi: toi.toi,
            normal: toi.normal1,
            point: toi.witness1,
        }
    }

    /// Use the second shape in the shape-cast as the cast result.
    pub fn from_toi2(toi: Toi) -> Self {
        Self {
            toi: toi.toi,
            normal: toi.normal2,
            point: toi.witness2,
        }
    }
}

impl From<RayIntersection> for CastResult {
    fn from(intersection: RayIntersection) -> Self {
        Self {
            toi: intersection.toi,
            normal: intersection.normal,
            point: intersection.point,
        }
    }
}

pub fn contact_manifolds(
    ctx: &RapierContext,
    position: Vec3,
    rotation: Quat,
    collider: &Collider,
    filter: &QueryFilter,
) -> Vec<(Entity, ContactManifold)> {
    const FUDGE: f32 = 0.05;

    let physics_scale = ctx.physics_scale();

    /*let RapierContext {
        query_pipeline, bodies, colliders, entity2collider, entity2body, ..
    } = &*ctx;*/

    let shape = &collider.raw;
    let shape_iso = Isometry3 {
        translation: (position * physics_scale).into(),
        rotation: rotation.into(),
    };

    let shape_aabb = shape.compute_aabb(&shape_iso).loosened(FUDGE);

    let mut manifolds = Vec::new();
    ctx.query_pipeline
        .colliders_with_aabb_intersecting_aabb(&shape_aabb, |handle| {
            if let Some(collider) = ctx.colliders.get(*handle) {
                if RapierContext::with_query_filter(&ctx, *filter, |rapier_filter| {
                    rapier_filter.test(&ctx.bodies, *handle, collider)
                }) {
                    let mut new_manifolds = Vec::new();
                    let pos12 = shape_iso.inv_mul(collider.position());
                    let _ = DefaultQueryDispatcher.contact_manifolds(
                        &pos12,
                        shape.as_ref(),
                        collider.shape(),
                        0.05,
                        &mut new_manifolds,
                        &mut None,
                    );

                    if let Some(entity) = ctx.collider_entity(*handle) {
                        manifolds.extend(new_manifolds.into_iter().map(|manifold| (entity, manifold)));
                    }
                }
            }

            true
        });

    manifolds
}

/// Robust casting to find the ground beneath the controller.
pub fn ground_cast(
    ctx: &RapierContext,
    colliders: &Query<&Collider>,
    globals: &Query<&GlobalTransform>,
    shape_pos: Vec3,
    shape_rot: Quat,
    shape_vel: Vec3,
    shape: &Collider,
    max_toi: f32,
    filter: QueryFilter,
    gizmos: &mut Gizmos,
) -> Option<(Entity, CastResult)> {
    const FUDGE: f32 = 0.05;

    let shape_vel = shape_vel.normalize_or_zero();
    let mut shape_pos = shape_pos + -shape_vel * FUDGE;
    let original_pos = shape_pos;
    gizmos.sphere(shape_pos, shape_rot, 0.25, Color::CYAN);

    // We are penetrating something, try to push the shape cast out from any contacts.
    let manifolds = contact_manifolds(ctx, shape_pos, shape_rot, shape, &filter);
    for (entity, manifold) in &manifolds {
        let local_normal: Vec3 = manifold.local_n2.into();

        let Ok(contact_global) = globals.get(*entity) else { continue };
        let normal = contact_global.to_scale_rotation_translation().1 * local_normal;
        for point in &manifold.points {
            let dist = point.dist;
            //if dist < 0.0 {
                //let correction = normal * -dist * 100.0;
                let correction = normal * 0.01;
                shape_pos += correction;
            //}
        }
    }

    //info!("corrections: {:?}", corrections);

    gizmos.sphere(shape_pos, shape_rot, 0.3, Color::BLUE);

    let height_dot = (shape_pos - original_pos).dot(-shape_vel);
    if height_dot > 0.0 {
        shape_pos += shape_vel * height_dot;
    }
    let Some((mut entity, toi)) =
            ctx.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter) else { return None };

    let casted_pos = shape_pos + shape_vel * toi.toi;
    let contact_dir = shape_rot * toi.normal2;
    /*
    let down_alignment = contact_dir.dot(shape_vel);
    if down_alignment <= FUDGE {
        let correction = -contact_dir * FUDGE;
        shape_pos += correction;
    }
    */

    if toi.status != TOIStatus::Penetrating && toi.toi > 0.0 {
        gizmos.ray(casted_pos, contact_dir * 0.5, Color::GREEN);
        //gizmos.sphere(toi.witness1, Quat::IDENTITY, 0.05, Color::GREEN);
        //gizmos.sphere(casted_pos, Quat::IDENTITY, 0.3, Color::PURPLE);
        //info!("toi: {:.2?}", toi.toi);
        //gizmos.sphere(shape_pos + shape_vel * toi.toi, Quat::IDENTITY, 0.3, Color::BLUE);
        let mut cast_result = CastResult::from_toi1(toi);
        cast_result.normal = Vec3::ZERO;
        //gizmos.sphere(cast_result.point, Quat::IDENTITY, 0.05, Color::CYAN);

        // try to get a better normal rather than an edge interpolated normal.
        // project back onto original shape position
        let above = toi.witness1 + -contact_dir * toi.toi;

        let dir = (shape_pos - above).normalize_or_zero();
        // nudge slightly in the direction of the hit point
        let nudged = above - dir * FUDGE;

        let direct_ray = (cast_result.point - shape_pos).normalize_or_zero();

        let (x, z) = shape_vel.any_orthonormal_pair();
        let samples = [-x + z, -x - z, x + z, x - z, Vec3::ZERO];

        // Initial correction, sample points around the contact point
        // for the closest normal
        let mut sampled = Vec::new();
        for sample in samples {
            if let Some((ray_entity, inter)) = ctx.cast_ray_and_get_normal(
                above - sample * FUDGE,
                contact_dir,
                max_toi,
                true,
                filter,
            ) {
                if inter.toi > 0.0
                    && inter.normal.length() > 0.0
                    && inter.point.distance(toi.witness1) < FUDGE * 2.0
                {
                    gizmos.ray(inter.point, inter.normal * 0.5, Color::RED);
                    sampled.push(inter.normal);
                }
            }
        }

        let mut sum = Vec3::ZERO;
        let len = sampled.len() as f32;
        for sample in sampled {
            sum += sample;
        }
        let average = sum / len;
        cast_result.normal = average;

        gizmos.ray(cast_result.point, cast_result.normal * 0.5, Color::RED);

        if cast_result.normal.length() > 0.0 {
            return Some((entity, cast_result));
        } else {
            warn!("normal is 0");
            return None;
        }
    }

    // Final attempt to check the ground by just raycasting downwards.
    // This should only occur if the controller fails to correct penetration
    // of colliders.

    // We need to offset it so the point of contact is identical to the shape cast.
    let offset = shape
        .cast_local_ray(Vec3::ZERO, shape_vel, 10.0, false)
        .unwrap_or(0.);
    shape_pos = shape_pos + shape_vel * offset;

    ctx.cast_ray_and_get_normal(shape_pos, shape_vel, max_toi, true, filter)
        .map(|(entity, inter)| (entity, inter.into()))
}
