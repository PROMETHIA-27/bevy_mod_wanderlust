use crate::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn movement(
    mut bodies: Query<(
        Entity,
        &Transform,
        &GlobalTransform,
        &mut ExternalImpulse,
        &mut CharacterController,
    )>,
    velocities: Query<&Velocity>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    ctx: Res<RapierContext>,
) {
    for (entity, tf, gtf, mut body, mut controller) in bodies.iter_mut() {
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
        let cast = ctx.cast_ray(
            gtf.translation,
            controller.settings.float_ray_dir,
            controller.settings.float_ray_length,
            true,
            default(),
            Some(&|collider| collider != entity),
        );

        let velocity = velocities
            .get(entity)
            .expect("Character controllers must have a Velocity");
        let ground_vel;

        // Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
        let float_spring = if let Some((ground, distance)) = cast {
            ground_vel = velocities.get(ground).ok();

            let vel_align = controller.settings.float_ray_dir.dot(velocity.linvel);
            let ground_vel_align = controller
                .settings
                .float_ray_dir
                .dot(ground_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO));

            let relative_align = vel_align - ground_vel_align;

            let snap = distance - controller.settings.float_distance;

            controller.settings.float_ray_dir
                * ((snap * controller.settings.float_strength)
                    - (relative_align * controller.settings.float_dampen))
        } else {
            ground_vel = None;
            Vec3::ZERO
        };

        // Calculate horizontal movement force
        let movement = {
            let unit_dir = if dir != Vec3::ZERO {
                dir.normalize()
            } else {
                Vec3::ZERO
            };

            // /*
            // let unit_vel = controller.last_goal_velocity.normalized();

            // let vel_dot = unit_dir.dot(unit_vel);
            // */
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
        let jump = if input.just_pressed(KeyCode::Space) {
            Vec3::Y * controller.settings.jump_force
        } else {
            Vec3::ZERO
        };

        // Apply force to the rigidbody
        body.impulse = movement + jump + float_spring;
    }
}
