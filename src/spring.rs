use bevy::{math::*, prelude::*};

/// Generic strength argument to springs and movement arguments.
#[derive(Debug, Clone, Reflect)]
pub enum Strength {
    /// Scaled by the mass and delta time.
    ///
    /// This effectively means the strength is just a function of 0..1
    /// where 1 is the goal and 0 is the start.
    Instant(f32),
    /// Scaled by the mass so this the behavior stays relatively the same
    /// regardless of the controller's mass
    ///
    /// You probably want to use this one over the other two.
    Scaled(f32),
    /// Unaffected force, this will be applied regardless of any
    /// other factors.
    Raw(f32),
}

impl Strength {
    pub fn get(&self, mass: f32, dt: f32) -> f32 {
        match *self {
            Self::Instant(raw) => raw * mass / dt,
            Self::Scaled(raw) => raw * mass,
            Self::Raw(raw) => raw,
        }
    }
}

pub enum SpringStrength {
    AngularFrequency(f32),
    StiffnessCoefficient(f32),
}

impl SpringStrength {
    pub fn get(&self, mass: f32, dt: f32) -> f32 {
        match self {
            Self::AngularFrequency(angular) => mass * angular * angular,
            Self::StiffnessCoefficient(raw) => *raw,
        }
    }
}

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
