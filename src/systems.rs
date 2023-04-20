use crate::components::{ControllerInput, ControllerSettings, ControllerState};
use crate::WanderlustPhysicsTweaks;
use bevy::ecs::system::SystemParam;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;

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
        ),
    >,
    velocities: Query<'w, 's, &'static Velocity>,
    globals: Query<'w, 's, &'static GlobalTransform>,
    masses: Query<'w, 's, &'static ReadMassProperties>,
    impulses: Query<'w, 's, &'static mut ExternalImpulse>,
    ctx: ResMut<'w, RapierContext>,
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// This system is useful for cases such as running on a fixed timestep.*
///
/// The system that controls movement logic.
pub fn movement(
    params: MovementParams,
    mut ground_casts: Local<Vec<(Entity, Toi)>>,

    #[cfg(feature = "debug_lines")] mut lines: ResMut<DebugLines>,
) {
    let MovementParams {
        mut bodies,
        velocities,
        globals,
        masses,
        mut impulses,
        ctx,
    } = params;

    for (entity, mut controller, settings, input) in bodies.iter_mut() {
        let mass_properties = masses
            .get(entity)
            .expect("character controllers must have a `ReadMassProperties` component");
        let tf = globals
            .get(entity)
            .expect("character controllers must have a `GlobalTransform` component");

        let dt = ctx.integration_parameters.dt;
        let mass = mass_properties.0.mass;
        let inertia = mass_properties.0.principal_inertia;
        let local_center_of_mass = mass_properties.0.local_center_of_mass;

        if !settings.valid() || dt == 0.0 {
            return;
        }

        // Get the ground and velocities
        let ground_cast = if controller.skip_ground_check_timer == 0.0
            && !settings.skip_ground_check_override
        {
            intersections_with_shape_cast(
                &ctx,
                ShapeDesc {
                    shape_pos: tf.transform_point(settings.float_cast_origin),
                    shape_rot: tf.to_scale_rotation_translation().1,
                    shape_vel: -settings.up_vector,
                    shape: &settings.float_cast_collider,
                },
                settings.float_cast_length,
                QueryFilter::new().exclude_sensors().predicate(&|collider| {
                    collider != entity && !settings.exclude_from_ground.contains(&collider)
                }),
                &mut ground_casts,
            );
            ground_casts
                .iter()
                .find(|(_, i)| {
                    i.status != TOIStatus::Penetrating
                        && i.normal1.angle_between(settings.up_vector) <= settings.max_ground_angle
                })
                .cloned()
        } else {
            controller.skip_ground_check_timer = (controller.skip_ground_check_timer - dt).max(0.0);
            None
        };

        // If we hit something, just get back up instead of waiting.
        if ctx.contacts_with(entity).next().is_some() {
            controller.skip_ground_check_timer = 0.0;
        }

        let float_offset = if let Some((_, toi)) = ground_cast {
            Some(toi.toi - settings.float_distance)
        } else {
            None
        };

        let grounded = float_offset
            .map(|offset| {
                offset <= settings.max_float_offset && offset >= settings.min_float_offset
            })
            .unwrap_or(false);

        if grounded {
            controller.remaining_jumps = settings.extra_jumps;
            controller.coyote_timer = settings.coyote_time_duration;
        } else {
            controller.coyote_timer = (controller.coyote_timer - dt).max(0.0);
        }

        // Gravity
        let gravity = if ground_cast.is_none() {
            settings.up_vector * mass * settings.gravity * dt
        } else {
            Vec3::ZERO
        };

        // Collect velocities
        let velocity = velocities
            .get(entity)
            .expect("character controllers must have a `Velocity` component");
        let ground_vel;

        // Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
        let float_spring_force = if let Some((ground, intersection)) = ground_cast {
            ground_vel = velocities.get(ground).ok();

            let point_velocity =
                velocity.linvel + velocity.angvel.cross(Vec3::ZERO - local_center_of_mass);
            let vel_align = (-settings.up_vector).dot(point_velocity);
            let ground_vel_align =
                (-settings.up_vector).dot(ground_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO));

            let relative_align = vel_align - ground_vel_align;

            let snap = intersection.toi - settings.float_distance;

            (-settings.up_vector)
                * ((snap * settings.float_spring.strength)
                    - (relative_align * settings.float_spring.damp_coefficient(mass)))
        } else {
            ground_vel = None;
            Vec3::ZERO
        };

        let mut float_spring = float_spring_force * dt;

        // Calculate horizontal movement force
        let movement = {
            let dir = input.movement.clamp_length_max(1.0);

            // let unit_vel = controller.last_goal_velocity.normalized();

            // let vel_dot = unit_dir.dot(unit_vel);

            let accel = settings.acceleration;

            let input_goal_vel = dir * settings.max_speed;

            let goal_vel = Vec3::lerp(
                controller.last_goal_velocity,
                input_goal_vel + ground_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO),
                (accel * dt).min(1.0),
            );

            let needed_accel = goal_vel - velocity.linvel;

            let max_accel_force = settings.max_acceleration_force;

            let needed_accel = needed_accel.clamp_length_max(max_accel_force);

            controller.last_goal_velocity = goal_vel;

            needed_accel * settings.force_scale
        };

        let just_jumped = input.jumping && !controller.jump_pressed_last_frame;
        if !grounded {
            if just_jumped {
                controller.jump_buffer_timer = settings.jump_buffer_duration;
            } else {
                controller.jump_buffer_timer = (controller.jump_buffer_timer - dt).max(0.0);
            }
        }

        // Calculate jump force
        let mut jump = if controller.jump_timer > 0.0 && !grounded {
            if !input.jumping {
                controller.jump_timer = 0.0;
                velocity.linvel.project_onto(settings.up_vector) * -settings.jump_stop_force
            } else {
                controller.jump_timer = (controller.jump_timer - dt).max(0.0);

                // Float force can lead to inconsistent jump power
                float_spring = Vec3::ZERO;

                settings.jump_force
                    * settings.up_vector
                    * (settings.jump_decay_function)(
                        (settings.jump_time - controller.jump_timer) / settings.jump_time,
                    )
                    * dt
            }
        } else {
            Vec3::ZERO
        };

        // Trigger a jump
        if (just_jumped || controller.jump_buffer_timer > 0.0)
            && (grounded || controller.coyote_timer > 0.0 || controller.remaining_jumps > 0)
        {
            if !grounded && controller.coyote_timer == 0.0 {
                controller.remaining_jumps -= 1;
            }

            controller.jump_buffer_timer = 0.0;
            controller.jump_timer = settings.jump_time;
            controller.skip_ground_check_timer = settings.jump_skip_ground_check_duration;
            // Negating the current velocity increases consistency for falling jumps,
            // and prevents stacking jumps to reach high upwards velocities
            jump = velocity.linvel * settings.up_vector * -1.0;
            jump += settings.jump_initial_force * settings.up_vector;
            // Float force can lead to inconsistent jump power
            float_spring = Vec3::ZERO;
        }

        // Calculate force to stay upright
        let upright = {
            let desired_axis = if let Some(forward) = settings.forward_vector {
                let right = settings.up_vector.cross(forward).normalize();
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
                current.cross(settings.up_vector)
            };

            let damping = Vec3::new(
                settings.upright_spring.damp_coefficient(inertia.x),
                settings.upright_spring.damp_coefficient(inertia.y),
                settings.upright_spring.damp_coefficient(inertia.z),
            );

            let spring =
                (desired_axis * settings.upright_spring.strength) - (velocity.angvel * damping);
            spring.clamp_length_max(settings.upright_spring.strength)
        };

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
                    lines.line_colored(
                        toi.witness1,
                        toi.witness1 + opposing_impulse,
                        dt,
                        Color::RED,
                    );
                }
            }
        }

        controller.jump_pressed_last_frame = input.jumping;
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
    collisions: &mut Vec<(Entity, Toi)>,
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

        if let Some(collision) =
            ctx.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter)
        {
            collisions.push(collision);
        } else {
            break;
        }
    }
}
