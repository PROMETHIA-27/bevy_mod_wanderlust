
use crate::controller::*;
use bevy::utils::HashSet;
use bevy_rapier3d::{na::Isometry3, prelude::*};

/// List of entities that are a part of this controller.
#[derive(Default, Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Parts {
    pub parts: Vec<Entity>,
}

impl Parts {
    pub fn parts(&self, controller: Entity) -> Vec<Entity> {
        let mut parts = self.parts.clone();
        parts.push(controller);
        parts
    }
}

#[derive(Default, Bundle)]
pub struct PartsBundle {
    /// Calculated force for allowing the controller to jump.
    pub jump_force: JumpForce,
    pub gravity_force: GravityForce,
    pub float_force: FloatForce,
    /// Physics bundle.
    pub physics: ControllerPhysicsBundle,
}
