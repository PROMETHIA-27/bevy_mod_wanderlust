use bevy::{math::*, prelude::*};

/// Spring parameters for a dampened harmonic oscillator.
///
/// Some good readings on this:
/// - https://www.ryanjuckett.com/damped-springs/
/// - https://gafferongames.com/post/spring_physics/
#[derive(Debug, Clone, Copy, Reflect)]
pub struct Spring {
    /// How strong the spring will push it into position.
    pub strength: f32,
    /// Damping ratio for the spring, this prevents endless oscillation if greater than 0.
    /// <1 is under-dampened so it will overshoot the target
    /// 1 is critically dampened so it will slow just enough to reach the target without overshooting
    /// >1 is over-dampened so it will reach the target slowly.
    pub damping: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            strength: 1.0,
            damping: 0.25,
        }
    }
}

impl Spring {
    /// The damping coefficient that will just reach the target without overshooting.
    pub fn critical_damping_point(&self, mass: f32) -> f32 {
        2.0 * (mass * self.strength).sqrt()
    }

    /// Get the correct damping coefficient for our damping ratio.
    /// See [`Spring`]'s damping for more information on the ratio.
    pub fn damp_coefficient(&self, mass: f32) -> f32 {
        self.damping * self.critical_damping_point(mass)
    }
}
