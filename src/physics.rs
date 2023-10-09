use bevy::prelude::*;

/// Force applied to the controller.
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerForce {
    /// Change in linear velocity.
    pub linear: Vec3,
    /// Change in angular velocity.
    pub angular: Vec3,
}

/// Components for computing forces/applying to physics engines.
#[derive(Bundle)]
pub struct ControllerPhysicsBundle {
    /// Accumulated force of various controller constraints.
    pub force: ControllerForce,
}

impl Default for ControllerPhysicsBundle {
    fn default() -> Self {
        Self {
            force: default(),
        }
    }
}
