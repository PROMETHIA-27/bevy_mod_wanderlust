use crate::controller::*;

/// How strong is the gravity for this controller.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration in the `up_vector` direction due to gravity.
    ///
    /// The default is `-9.817`, but for most games it is recommended to
    /// use a higher acceleration. The reasoning being that normal/reality-based
    /// gravity tends to feel floaty.
    pub acceleration: f32,
    /// Direction we should float up from.
    ///
    /// The default is `Vec3::Y`.
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
pub fn gravity_force(mut query: Query<(&mut GravityForce, &Gravity, &ControllerMass)>) {
    for (mut force, gravity, mass) in &mut query {
        force.linear = gravity.up_vector * mass.mass * gravity.acceleration;
    }
}
