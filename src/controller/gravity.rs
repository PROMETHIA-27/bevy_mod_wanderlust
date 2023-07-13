use crate::controller::*;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration due to gravity
    pub acceleration: Vec3,
    /// Normalized negative acceleration
    pub up_vector: Vec3,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GravityForce {
    pub linear: Vec3,
}

pub fn apply_gravity(
    mut query: Query<(&mut GravityForce, &Gravity, &GroundCast, &ControllerMass)>,
) {
    for (mut force, gravity, ground, mass) in &mut query {
        force.linear = if ground.cast.is_none() {
            gravity.up_vector * mass.mass * gravity.acceleration
        } else {
            Vec3::ZERO
        };
    }
}
