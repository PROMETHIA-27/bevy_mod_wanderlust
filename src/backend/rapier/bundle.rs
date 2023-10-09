
use bevy::prelude::*;
use super::rapier::prelude::*;

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct RapierPhysicsBundle {
    /// See [`RigidBody`].
    pub rigidbody: RigidBody,
    /// See [`Collider`].
    pub collider: Collider,
    /// See [`Velocity`].
    pub velocity: Velocity,
    /// See [`GravityScale`].
    pub gravity: GravityScale,
    /// See [`Sleeping`].
    pub sleeping: Sleeping,
    /// See [`Ccd`].
    pub ccd: Ccd,
    /// See [`ExternalImpulse`].
    pub force: ExternalImpulse,
    /// See [`LockedAxes`].
    pub locked_axes: LockedAxes,
    /// See [`Friction`].
    pub friction: Friction,
    /// See [`Damping`].
    pub damping: Damping,
    /// See [`Restitution`].
    pub restitution: Restitution,
    /// See [`ReadMassProperties`].
    pub read_mass_properties: ReadMassProperties,
}

impl Default for RapierPhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: default(),
            collider: Collider::capsule(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.5, 0.0), 0.5),
            velocity: default(),
            gravity: GravityScale(0.0),
            sleeping: default(),
            ccd: default(),
            force: default(),
            locked_axes: default(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            read_mass_properties: default(),
        }
    }
}
