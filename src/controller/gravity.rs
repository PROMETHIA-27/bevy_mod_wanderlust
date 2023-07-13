use crate::controller::*;

/// How strong is the gravity for this controller.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration in the `up_vector` direction due to gravity.
    pub acceleration: f32,
    /// Direction we should float up from.
    pub up_vector: Vec3,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity {
            acceleration: -9.817,
            up_vector: Vec3::Y,
        }
    }
}

/// Calculated gravity force.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GravityForce {
    /// Linear gravitational force.
    pub linear: Vec3,
}

/// Calculate gravity force.
pub fn gravity_force(
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
