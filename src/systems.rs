use std::ops::Neg;

use crate::components::{
    ContinuousMovement, ControllerInput, ControllerSettings, ControllerState, CoyoteTime,
    ExtraJumps, FinalMotion, Float, FloatForce, Gravity, GroundCast, GroundCaster, Grounded,
    Jumping, Mass, UprightForce, UprightSpring, Velocity,
};
use crate::WanderlustPhysicsTweaks;
use bevy::ecs::system::SystemParam;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(SystemParam)]
pub struct MovementParams<'w, 's> {
    bodies: Query<
        'w,
        's,
        (
            Entity,
            &'static mut ControllerState,
            &'static ControllerSettings,
            &'static mut ControllerInput,
            &'static mut GroundCast,
        ),
    >,
    velocities: Query<'w, 's, &'static Velocity>,
    globals: Query<'w, 's, &'static GlobalTransform>,
    masses: Query<'w, 's, &'static ReadMassProperties>,
    impulses: Query<'w, 's, &'static mut ExternalImpulse>,
    ctx: ResMut<'w, RapierContext>,
}

/* Setup phase */

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

fn create_float_force(
    mut c: Commands,
    forceless_floaters: Query<Entity, (With<Float>, Without<FloatForce>)>,
) {
    for ent in &forceless_floaters {
        c.entity(ent).insert(FloatForce::default());
    }
}

fn create_upright_force(
    mut c: Commands,
    forceless_uprighters: Query<Entity, (With<UprightSpring>, Without<UprightForce>)>,
) {
    for ent in &forceless_uprighters {
        c.entity(ent).insert(UprightForce::default());
    }
}

/// Cache the up vector as the normalized negation of acceleration due to gravity.
pub fn set_up_vector(query: Query<&mut Gravity>) {
    for gravity in &mut query {
        gravity.up_vector = gravity.acceleration.neg().normalize_or_zero();
    }
}

pub fn get_mass_from_rapier(query: Query<(&mut Mass, &ReadMassProperties)>) {
    for (mass, rapier_mass) in &mut query {
        mass.mass = rapier_mass.0.mass;
        mass.inertia = rapier_mass.0.principal_inertia;
        mass.com = rapier_mass.0.local_center_of_mass;
    }
}

pub fn get_velocity_from_rapier(query: Query<(&mut Velocity, &bevy_rapier3d::prelude::Velocity)>) {
    for (vel, rapier_vel) in &mut query {
        vel.lin = rapier_vel.linvel;
        vel.ang = rapier_vel.angvel;
    }
}

/* Action phase */

/// Performs groundcasting and updates controller state accordingly.
pub fn find_ground(
    casters: Query<(
        Entity,
        &GlobalTransform,
        &GroundCaster,
        &Gravity,
        &mut GroundCast,
    )>,
    velocities: Query<&Velocity>,
    ctx: Res<RapierContext>,
    mut ground_casts: Local<Vec<(Entity, Toi, Velocity)>>,
) {
    let dt = ctx.integration_parameters.dt;
    for (entity, tf, caster, gravity, cast) in casters.iter() {
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
                velocities,
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

pub fn determine_groundedness(query: Query<(&Float, &GroundCast, &mut Grounded)>) {
    for (float, cast, grounded) in &mut query {
        let float_offset = if let Some((_, toi, _)) = cast.cast {
            Some(toi.toi - float.distance)
        } else {
            None
        };

        let grounded = float_offset
            .map(|offset| offset <= float.max_offset && offset >= float.min_offset)
            .unwrap_or(false);
    }
}

pub fn reset_jumps(query: Query<(&mut ExtraJumps, &Grounded)>) {
    for (jumps, grounded) in &mut query {
        if grounded.grounded {
            jumps.remaining = jumps.extra;
        }
    }
}

pub fn tick_coyote_timer(query: Query<(&mut CoyoteTime, &Grounded)>, ctx: Res<RapierContext>) {
    let dt = ctx.integration_parameters.dt;
    for (coyote, grounded) in &mut query {
        if grounded.grounded {
            coyote.timer = coyote.duration;
        } else {
            coyote.timer = (coyote.timer - dt).max(0.0);
        }
    }
}

pub fn get_gravity_contribution(
    query: Query<(
        &mut FinalMotion,
        &Gravity,
        Option<&GroundCast>,
        Option<&Mass>,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (motion, gravity, ground, mass) in &mut query {
        let mass = mass.map(|mass| mass.mass).unwrap_or(1.0);
        let gravity = if ground.map(|ground| ground.cast.is_none()).unwrap_or(false) {
            gravity.up_vector * mass * gravity.acceleration * dt
        } else {
            Vec3::ZERO
        };
    }
}

/// Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
pub fn determine_float_force(
    query: Query<(
        &mut FloatForce,
        &Float,
        &GroundCast,
        Option<&Velocity>,
        Option<&Mass>,
        Option<&Gravity>,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (force, float, cast, velocity, mass, gravity) in &mut query {
        let float_spring_force = if let Some((ground, intersection, ground_vel)) = cast.cast {
            let velocity = velocity.copied().unwrap_or_default();
            let com = mass.map(|mass| mass.com).unwrap_or_default();
            let mass = mass.map(|mass| mass.mass).unwrap_or(1.0);
            let up_vector = gravity
                .map(|grav| grav.up_vector)
                .unwrap_or(intersection.normal1);

            let point_velocity = velocity.lin + velocity.ang.cross(Vec3::ZERO - com);
            let vel_align = (-up_vector).dot(point_velocity);
            let ground_vel_align = (-up_vector).dot(ground_vel.lin);

            let relative_align = vel_align - ground_vel_align;

            let snap = intersection.toi - float.distance;

            (-up_vector)
                * ((snap * float.spring.strength)
                    - (relative_align * float.spring.damp_coefficient(mass)))
        } else {
            Vec3::ZERO
        };

        force.force = float_spring_force * dt;
    }
}

pub fn determine_continuous_movement(
    query: Query<(
        &mut FinalMotion,
        &ContinuousMovement,
        &ControllerInput,
        &GroundCast,
        Option<&Velocity>,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (motion, movement, input, ground, velocity) in &mut query {
        let movement = {
            let velocity = velocity.copied().unwrap_or_default();

            let dir = input.movement.clamp_length_max(1.0);

            // let unit_vel = controller.last_goal_velocity.normalized();

            // let vel_dot = unit_dir.dot(unit_vel);

            let accel = movement.acceleration;

            let input_rel_goal = dir * movement.max_speed;

            // let goal_vel = Vec3::lerp(
            //     controller.last_goal_velocity,
            //     input_goal_vel + ground.cast.map(|cast| cast.2.lin).unwrap_or(Vec3::ZERO),
            //     (accel * dt).min(1.0),
            // );

            // let needed_accel = goal_vel - velocity.linvel;

            let ground_vel = ground.cast.map(|(_, _, vel)| vel.lin).unwrap_or_default();

            let rel_vel = velocity.lin - ground_vel;

            let needed_accel = input_rel_goal - rel_vel;

            let max_accel_force = movement.max_acceleration_force;

            let needed_accel = needed_accel.clamp_length_max(max_accel_force);

            // controller.last_goal_velocity = goal_vel;

            // needed_accel * settings.force_scale
            needed_accel
        };
        motion.internal += movement;
        motion.total += movement;
    }
}

pub fn determine_jump_contribution(
    query: Query<(
        &mut Jumping,
        &mut FloatForce,
        &mut GroundCaster,
        &ControllerInput,
        &Grounded,
        &Gravity,
        &Velocity,
        Option<&mut ExtraJumps>,
        Option<&CoyoteTime>,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (
        mut jumping,
        mut float_force,
        mut groundcaster,
        input,
        grounded,
        gravity,
        velocity,
        mut extra_jumps,
        coyote,
    ) in &mut query
    {
        let grounded = grounded.grounded;
        let just_jumped = input.jumping && !jumping.jump_pressed_last_frame;
        if !grounded {
            if just_jumped {
                jumping.jump_buffer_timer = jumping.jump_buffer_duration;
            } else {
                jumping.jump_buffer_timer = (jumping.jump_buffer_timer - dt).max(0.0);
            }
        }

        // Calculate jump force
        let mut jump = if jumping.jump_timer > 0.0 && !grounded {
            if !input.jumping {
                jumping.jump_timer = 0.0;
                velocity.lin.project_onto(gravity.up_vector) * -jumping.jump_stop_force
            } else {
                jumping.jump_timer = (jumping.jump_timer - dt).max(0.0);

                // Float force can lead to inconsistent jump power
                float_force.force = Vec3::ZERO;

                jumping.jump_force
                    * gravity.up_vector
                    * jumping
                        .jump_decay_function
                        .map(|f| (f)((jumping.jump_time - jumping.jump_timer) / jumping.jump_time))
                        .unwrap_or(1.0)
                    * dt
            }
        } else {
            Vec3::ZERO
        };

        // Trigger a jump
        let coyote_timer = coyote.map(|cy| cy.timer).unwrap_or(0.0);
        let remaining_jumps = extra_jumps.map(|ej| ej.remaining).unwrap_or(0);
        if (just_jumped || jumping.jump_buffer_timer > 0.0)
            && (grounded || coyote_timer > 0.0 || remaining_jumps > 0)
        {
            if !grounded && coyote_timer == 0.0 {
                if let Some(ej) = extra_jumps {
                    ej.remaining -= 1;
                }
            }

            jumping.jump_buffer_timer = 0.0;
            jumping.jump_timer = jumping.jump_time;
            groundcaster.skip_ground_check_timer = jumping.jump_skip_ground_check_duration;
            // Negating the current velocity increases consistency for falling jumps,
            // and prevents stacking jumps to reach high upwards velocities
            jump = velocity.lin * gravity.up_vector * -1.0;
            jump += jumping.jump_initial_force * gravity.up_vector;
            // Float force can lead to inconsistent jump power
            float_force.force = Vec3::ZERO;
        }
    }
}

pub fn determine_upright_force(
    query: Query<(
        &mut UprightForce,
        &UprightSpring,
        &GlobalTransform,
        &Gravity,
        &Mass,
        &Velocity,
    )>,
) {
    for (force, upright, tf, gravity, mass, velocity) in &query {
        force.force = {
            let desired_axis = if let Some(forward) = upright.forward_vector {
                let right = gravity.up_vector.cross(forward).normalize();
                let up = forward.cross(right);
                let target_rot = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
                let current = tf.to_scale_rotation_translation().1;
                let rot = target_rot * current.inverse();
                let (axis, mut angle) = rot.to_axis_angle();
                if angle > std::f32::consts::PI {
                    angle -= 2.0 * std::f32::consts::PI;
                }
                axis * angle
            } else {
                let current = tf.up();
                current.cross(gravity.up_vector)
            };

            let damping = Vec3::new(
                upright.upright_spring.damp_coefficient(mass.inertia.x),
                upright.upright_spring.damp_coefficient(mass.inertia.y),
                upright.upright_spring.damp_coefficient(mass.inertia.z),
            );

            let spring =
                (desired_axis * upright.upright_spring.strength) - (velocity.ang * damping);
            spring.clamp_length_max(upright.upright_spring.strength)
        };
    }
}

pub fn apply_forces(query: Query<()>) {
    for () in &query {
        let pushing_impulse = jump + float_spring + gravity;
        let total_impulse = movement + pushing_impulse;
        let opposing_impulse = -(movement * settings.opposing_movement_impulse_scale
            + pushing_impulse * settings.opposing_impulse_scale);

        if let Ok(mut body_impulse) = impulses.get_mut(entity) {
            // Apply positional force to the rigidbody
            body_impulse.impulse += total_impulse;
            // Apply rotational force to the rigidbody
            body_impulse.torque_impulse += upright * dt;
        }
    }
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// This system is useful for cases such as running on a fixed timestep.*
///
/// The system that controls movement logic.
pub fn movement(
    params: MovementParams,
    mut ground_casts: Local<Vec<(Entity, Toi)>>,

    #[cfg(feature = "debug_lines")] mut gizmos: Gizmos,
) {
    let MovementParams {
        mut bodies,
        velocities,
        globals,
        masses,
        mut impulses,
        ctx,
    } = params;

    for (entity, mut controller, settings, input, ground_cast) in bodies.iter_mut() {
        // Things we do per iter:
        // - ground cast to find certain info
        // - if we hit something, reset ground timer
        // - determine groundedness
        // - determine jumps/coyote time
        // - determine contribution from gravity
        // - get velocity
        // - determine float_spring force
        // - calculate continuous movement input contribution
        // - determine jump contribution
        // - calculate upright force
        // ----- Finished line
        // - apply forces
        // - apply opposite forces to things being stood on

        // Opposite force to whatever we were touching
        if let Some((ground_entity, toi)) = ground_cast {
            if toi.status != TOIStatus::Penetrating {
                if let Ok(mut ground_impulse) = impulses.get_mut(ground_entity) {
                    let ground_transform = match globals.get(ground_entity) {
                        Ok(global) => global.compute_transform(),
                        _ => Transform::default(),
                    };

                    let local_center_of_mass = match masses.get(ground_entity) {
                        Ok(properties) => properties.0.local_center_of_mass,
                        _ => Vec3::ZERO,
                    };

                    let center_of_mass = ground_transform * local_center_of_mass;

                    let push_impulse =
                        ExternalImpulse::at_point(opposing_impulse, toi.witness1, center_of_mass);
                    *ground_impulse += push_impulse;

                    #[cfg(feature = "debug_lines")]
                    {
                        let color = if opposing_impulse.dot(settings.up_vector) < 0.0 {
                            Color::RED
                        } else {
                            Color::BLUE
                        };
                        gizmos.line(toi.witness1, toi.witness1 + opposing_impulse, color);
                    }
                }
            }
        }

        controller.jump_pressed_last_frame = input.jumping;
        controller.is_grounded = grounded;
    }
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// Alternatively, if one only wants to disable the system, use [`WanderlustPhysicsTweaks`](WanderlustPhysicsTweaks).*
///
/// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
pub fn setup_physics_context(
    mut ctx: ResMut<RapierContext>,
    should_change: Option<Res<WanderlustPhysicsTweaks>>,
) {
    if should_change.map(|s| s.should_do_tweaks()).unwrap_or(true) {
        let params = &mut ctx.integration_parameters;
        // This prevents any noticeable jitter when running facefirst into a wall.
        params.erp = 0.99;
        // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
        params.max_velocity_iterations = 16;
        // TODO: Fix jitter that occurs when running facefirst into a normal corner.
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
    velocities: Query<&Velocity>,
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
