use super::components::*;
use bevy::prelude::*;
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
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            rigidbody: default(),
            collider: Collider::capsule_y(0.5, 0.5),
            transform: default(),
            velocity: default(),
            gravity: default(),
            sleeping: default(),
            ccd: default(),
        }
    }
}
