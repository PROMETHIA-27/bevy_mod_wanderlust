use crate::CharacterController;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn movement(
    mut bodies: Query<(
        Entity,
        &GlobalTransform,
        &mut ExternalImpulse,
        &mut CharacterController,
    )>,
    velocities: Query<&Velocity>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    ctx: Res<RapierContext>,
) {
    for (entity, tf, mut body, mut controller) in bodies.iter_mut() {
        let dt = time.delta_seconds();

        // Sometimes, such as at the beginning of the game, deltatime is 0. This
        // can cause division by 0 so I just skip those frames. A better solution
        // is a fixed framerate that has a static dt, but bevy doesn't have
        // that to my knowledge.
        if dt == 0.0 {
            return;
        }

        // Collect movement input vector
        let mut dir = Vec3::default();

        if input.pressed(KeyCode::W) {
            dir += tf.forward();
        }
        if input.pressed(KeyCode::S) {
            dir += -tf.forward();
        }
        if input.pressed(KeyCode::D) {
            dir += tf.right();
        }
        if input.pressed(KeyCode::A) {
            dir += -tf.right();
        }

        // Get the ground and velocities
        let ground_cast = if controller.skip_ground_check_timer == 0.0 {
            ctx.cast_shape(
                tf.mul_vec3(controller.settings.float_cast_origin),
                tf.rotation,
                -controller.settings.up_vector,
                &controller.settings.float_cast_collider,
                controller.settings.float_cast_length,
                QueryFilter::new().predicate(&|collider| collider != entity),
            )
            .filter(|(_, i)| {
                i.status != TOIStatus::Penetrating
                    && i.normal1.angle_between(controller.settings.up_vector)
                        <= controller.settings.max_ground_angle
            })
        } else {
            controller.skip_ground_check_timer = (controller.skip_ground_check_timer - dt).max(0.0);
            None
        };

        // Gravity
        let gravity = if ground_cast.is_none() {
            controller.settings.up_vector * -controller.settings.gravity * dt
        } else {
            Vec3::ZERO
        };

        // Collect velocities
        let velocity = velocities
            .get(entity)
            .expect("Character controllers must have a Velocity component");
        let ground_vel;

        // Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
        let mut float_spring = if let Some((ground, intersection)) = ground_cast {
            ground_vel = velocities.get(ground).ok();

            let vel_align = (-controller.settings.up_vector).dot(velocity.linvel);
            let ground_vel_align = (-controller.settings.up_vector)
                .dot(ground_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO));

            let relative_align = vel_align - ground_vel_align;

            let snap = intersection.toi - controller.settings.float_distance;

            (-controller.settings.up_vector)
                * ((snap * controller.settings.float_strength)
                    - (relative_align * controller.settings.float_dampen))
        } else {
            ground_vel = None;
            Vec3::ZERO
        };

        // Calculate horizontal movement force
        let movement = {
            let unit_dir = dir.normalize_or_zero();

            // let unit_vel = controller.last_goal_velocity.normalized();

            // let vel_dot = unit_dir.dot(unit_vel);

            let accel = controller.settings.acceleration;

            let input_goal_vel = unit_dir * controller.settings.max_speed;

            let goal_vel = Vec3::lerp(
                controller.last_goal_velocity,
                input_goal_vel + ground_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO),
                accel * dt,
            );

            let needed_accel = goal_vel - velocity.linvel;

            let max_accel_force = controller.settings.max_acceleration_force;

            let needed_accel = needed_accel.clamp_length_max(max_accel_force);

            controller.last_goal_velocity = goal_vel;

            needed_accel * controller.settings.force_scale
        };

        // Calculate jump force
        let mut jump = if controller.jump_timer > 0.0 && ground_cast.is_none() {
            controller.jump_timer = (controller.jump_timer - dt).max(0.0);

            // Float force can lead to inconsistent jump power
            float_spring = Vec3::ZERO;

            controller.settings.jump_force
                * controller.settings.up_vector
                * dt
                * (controller.settings.jump_decay_function)(
                    (controller.settings.jump_time - controller.jump_timer)
                        / controller.settings.jump_time,
                )
        } else {
            Vec3::ZERO
        };

        if input.just_pressed(KeyCode::Space) && ground_cast.is_some() {
            controller.jump_timer = controller.settings.jump_time;
            controller.skip_ground_check_timer =
                controller.settings.jump_skip_ground_check_duration;
            // Negating the current velocity increases consistency for falling jumps,
            // and prevents stacking jumps to reach high upwards velocities
            jump = velocity.linvel * controller.settings.up_vector * -1.0;
            jump += controller.settings.jump_initial_force * controller.settings.up_vector;
            // Float force can lead to inconsistent jump power
            float_spring = Vec3::ZERO;
        }

        // Calculate force to stay upright
        let upright = {
            let (to_goal_axis, to_goal_angle) = {
                let current = tf.up();
                (
                    current
                        .cross(controller.settings.up_vector)
                        .normalize_or_zero(),
                    current.angle_between(controller.settings.up_vector),
                )
            };

            ((to_goal_axis * (to_goal_angle * controller.settings.upright_spring_strength))
                - (velocity.angvel * controller.settings.upright_spring_damping))
                * dt
        };

        // Apply positional force to the rigidbody
        body.impulse = movement + jump + float_spring + gravity;
        // Apply rotational force to the rigidbody
        body.torque_impulse = upright;
    }
}
