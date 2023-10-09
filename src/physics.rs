use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Deref)]
pub struct PhysicsDeltaTime(pub f32);

/// Force applied to the controller.
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerForce {
    /// Change in linear velocity.
    pub linear: Vec3,
    /// Change in angular velocity.
    pub angular: Vec3,
}

/// Mass of the controller
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerMass {
    /// Mass of the controller.
    pub mass: f32,
    /// Principal inertia of the controller.
    pub inertia: Vec3,
    /// Local center of mass of the controller.
    pub local_center_of_mass: Vec3,
}

/// Velocity of the controller
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerVelocity {
    /// Linear velocity of the controller.
    pub linear: Vec3,
    /// Angular velocity of the controller.
    pub angular: Vec3,
}

/// Components for computing forces/applying to physics engines.
#[derive(Bundle)]
pub struct ControllerPhysicsBundle {
    /// Accumulated force of various controller constraints.
    pub force: ControllerForce,
    /// Mass of the controller
    pub mass: ControllerMass,
    /// Velocity of the controller
    pub velocity: ControllerVelocity,
}

impl Default for ControllerPhysicsBundle {
    fn default() -> Self {
        Self {
            force: default(),
            mass: default(),
            velocity: default(),
        }
    }
}
