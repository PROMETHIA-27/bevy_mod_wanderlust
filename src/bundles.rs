use super::components::*;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    controller: CharacterController,
    rigidbody: RigidBody,
    collider: Collider,
    transform: Transform,
    velocity: Velocity,
    gravity: GravityScale,
    sleeping: Sleeping,
    ccd: Ccd,
    force: ExternalImpulse,
    locked_axes: LockedAxes,
    friction: Friction,
    damping: Damping,
    restitution: Restitution,
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self {
            controller: CharacterController {
                settings: ControllerSettings {
                    acceleration: 50.0,
                    max_speed: 10.0,
                    max_acceleration_force: 10.0,
                    up_vector: Vec3::Y,
                    gravity: 25.0,
                    max_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
                    jump_force: 15.0,
                    jump_skip_ground_check_duration: 0.5,
                    force_scale: vec3(1.0, 0.0, 1.0),
                    float_cast_length: 2.0,
                    float_cast_origin: vec3(0.0, 0.25, 0.0),
                    float_cast_collider: Collider::ball(0.45),
                    float_distance: 1.0,
                    float_strength: 7.5,
                    float_dampen: 0.5,
                    upright_spring_strength: 5.0,
                    upright_spring_damping: 0.5,
                },
                ..default()
            },
            rigidbody: default(),
            collider: Collider::capsule(vec3(0.0, 0.25, 0.0), vec3(0.0, 0.5, 0.0), 0.5),
            transform: default(),
            velocity: default(),
            gravity: GravityScale(0.0),
            sleeping: default(),
            ccd: default(),
            force: default(),
            locked_axes: default(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}
