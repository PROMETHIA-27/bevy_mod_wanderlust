use bevy::prelude::*;

/// Mass/inertia properties for controller.
#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerMass {
    /// The mass of a character
    pub mass: f32,
    /// The rotational inertia of a character
    pub inertia: Vec3,
    /// The center of mass of a character
    pub com: Vec3,
}

/// Current velocity of the controller.
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ControllerVelocity {
    /// How fast this character is currently moving linearly in 3D space
    pub linear: Vec3,
    /// How fast this character is currently moving angularly in 3D space
    pub angular: Vec3,
}

/// Previous velocity of the controller.
#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct PreviousControllerVelocity(pub ControllerVelocity);

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
    /// Mass of the controller.
    pub mass: ControllerMass,
    /// Current velocity of the controller.
    pub velocity: ControllerVelocity,
    /// Previous velocity of the controller.
    pub previous_velocity: PreviousControllerVelocity,
    /// Accumulated force of various controller constraints.
    pub force: ControllerForce,
}

impl Default for ControllerPhysicsBundle {
    fn default() -> Self {
        Self {
            mass: default(),
            velocity: default(),
            previous_velocity: default(),
            force: default(),
        }
    }
}
