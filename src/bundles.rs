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
    axes: LockedAxes,
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
                    jump_force: 25.0,
                    float_ray_length: 1.0,
                    float_distance: 0.5,
                    float_ray_dir: -Vec3::Y,
                    float_strength: 10.0,
                    float_dampen: 0.5,
                },
            },
            rigidbody: default(),
            collider: Collider::compound(vec![(
                vec3(0.0, 0.25, 0.0),
                default(),
                Collider::capsule_y(0.25, 0.5),
            )]),
            transform: default(),
            velocity: default(),
            gravity: GravityScale(5.0),
            sleeping: default(),
            ccd: default(),
            force: default(),
            axes: LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::ROTATION_LOCKED_Z,
            friction: Friction {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 0.0,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}
