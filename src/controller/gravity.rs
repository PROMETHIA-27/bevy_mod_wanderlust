use crate::controller::*;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration due to gravity
    pub acceleration: Vec3,
}

impl Gravity {
    pub fn up_vector(&self) -> Vec3 {
        (-self.acceleration).normalize_or_zero()
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity {
            acceleration: Vec3::new(0.0, -9.817, 0.0),
        }
    }
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
            mass.mass * gravity.acceleration
        } else {
            Vec3::ZERO
        };
    }
}
