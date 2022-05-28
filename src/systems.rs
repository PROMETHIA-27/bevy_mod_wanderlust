use crate::CharacterController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn movement(
    mut bodies: Query<(
        Entity,
        &Transform,
        &GlobalTransform,
        &mut ExternalImpulse,
        &CharacterController,
    )>,
    velocities: Query<&Velocity>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    ctx: Res<RapierContext>,
) {
    for (entity, tf, gtf, mut body, controller) in bodies.iter_mut() {
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

        // Calculate horizontal movement force
        let movement = if dir.length() != 0.0 {
            dir.normalize() * time.delta_seconds() * controller.settings.acceleration
        } else {
            Vec3::ZERO
        };

        // Calculate jump force
        let jump = if input.just_pressed(KeyCode::Space) {
            Vec3::Y * controller.settings.jump_force
        } else {
            Vec3::ZERO
        };

        // Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
        let cast = ctx.cast_ray(
            gtf.translation,
            controller.settings.float_ray_dir,
            controller.settings.float_ray_length,
            true,
            default(),
            Some(&|collider| collider != entity),
        );
        let float_spring = if let Some((ground, distance)) = cast {
            let vel = velocities.get(entity).unwrap();
            let other_vel = velocities.get(ground).ok();

            let vel_align = controller.settings.float_ray_dir.dot(vel.linvel);
            let other_vel_align = controller
                .settings
                .float_ray_dir
                .dot(other_vel.map(|v| v.linvel).unwrap_or(Vec3::ZERO));

            let relative_align = vel_align - other_vel_align;

            let snap = distance - controller.settings.float_distance;

            controller.settings.float_ray_dir
                * ((snap * controller.settings.float_strength)
                    - (relative_align * controller.settings.float_dampen))
        } else {
            Vec3::ZERO
        };

        // Apply force to the rigidbody
        body.impulse = movement + jump + float_spring;
    }
}
