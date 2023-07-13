use crate::controller::*;
use bevy::{prelude::*, utils::HashSet};
use bevy_rapier3d::prelude::*;

#[derive(Component, Default, Reflect)]
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

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GroundCast {
    /// The cached ground cast. Contains the entity hit, the hit info, and velocity of the entity
    /// hit.
    #[reflect(ignore)]
    pub cast: Option<(Entity, Toi, Velocity)>,
}

/// Is the character grounded?
#[derive(Component, Default, Reflect, Deref)]
#[reflect(Component, Default)]
pub struct Grounded(pub bool);

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
    ctx: Res<RapierContext>,
    mut ground_casts: Local<Vec<(Entity, Toi, Velocity)>>,
) {
    let dt = ctx.integration_parameters.dt;
    for (entity, tf, gravity, mut caster, mut cast) in &mut casters {
        cast.cast = if caster.skip_ground_check_timer == 0.0 && !caster.skip_ground_check_override {
            intersections_with_shape_cast(
                &ctx,
                ShapeDesc {
                    shape_pos: tf.transform_point(caster.cast_origin),
                    shape_rot: tf.to_scale_rotation_translation().1,
                    shape_vel: -gravity.up_vector,
                    shape: &caster.cast_collider,
                },
                caster.cast_length,
                QueryFilter::new().exclude_sensors().predicate(&|collider| {
                    collider != entity && !caster.exclude_from_ground.contains(&collider)
                }),
                &mut ground_casts,
                &velocities,
            );
            ground_casts
                .iter()
                .find(|(_, i, _)| {
                    i.status != TOIStatus::Penetrating
                        && i.normal1.angle_between(gravity.up_vector) <= caster.max_ground_angle
                })
                .cloned()
        } else {
            caster.skip_ground_check_timer = (caster.skip_ground_check_timer - dt).max(0.0);
            None
        };

        // If we hit something, just get back up instead of waiting.
        if ctx.contacts_with(entity).next().is_some() {
            caster.skip_ground_check_timer = 0.0;
        }
    }
}

pub fn determine_groundedness(mut query: Query<(&Float, &GroundCast, &mut Grounded)>) {
    for (float, cast, mut grounded) in &mut query {
        let float_offset = if let Some((_, toi, _)) = cast.cast {
            Some(toi.toi - float.distance)
        } else {
            None
        };

        grounded.0 = float_offset
            .map(|offset| offset <= float.max_offset && offset >= float.min_offset)
            .unwrap_or(false);
    }
}

fn create_ground_cast(
    mut c: Commands,
    castless_casters: Query<Entity, (With<GroundCaster>, Without<GroundCast>)>,
) {
    for ent in &castless_casters {
        c.entity(ent).insert(GroundCast::default());
    }
}

fn create_grounded(
    mut c: Commands,
    groundless_casters: Query<Entity, (With<GroundCaster>, Without<Grounded>)>,
) {
    for ent in &groundless_casters {
        c.entity(ent).insert(Grounded::default());
    }
}

struct ShapeDesc<'c> {
    shape_pos: Vec3,
    shape_rot: Quat,
    shape_vel: Vec3,
    shape: &'c Collider,
}

fn intersections_with_shape_cast(
    ctx: &RapierContext,
    shape: ShapeDesc,
    max_toi: f32,
    filter: QueryFilter,
    collisions: &mut Vec<(Entity, Toi, Velocity)>,
    velocities: &Query<&Velocity>,
) {
    collisions.clear();

    let orig_predicate = filter.predicate;

    loop {
        let predicate = |entity| {
            !collisions.iter().any(|coll| coll.0 == entity)
                && orig_predicate.map(|pred| pred(entity)).unwrap_or(true)
        };
        let filter = filter.predicate(&predicate);

        let ShapeDesc {
            shape_pos,
            shape_rot,
            shape_vel,
            shape,
        } = shape;

        if let Some((entity, toi)) =
            ctx.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter)
        {
            let velocity = velocities.get(entity).copied().unwrap_or_default();
            collisions.push((entity, toi, velocity));
        } else {
            break;
        }
    }
}
