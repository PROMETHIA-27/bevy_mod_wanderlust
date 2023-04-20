use crate::{ControllerInput, ControllerSettings, ControllerState};

use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct ControllerPhysicsBundle {
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

impl Default for ControllerPhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: default(),
            collider: Collider::capsule(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.5, 0.0), 0.5),
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

/// The recommended bundle for creating a character controller. Includes the necessary components for a character controller
/// as well as many physics-related components that can be used to tweak the behavior of the controller. Try using the
/// [`Self::builder()`] method to construct the bundle!
#[derive(Bundle)]
pub struct ControllerBundle {
    /// See [`ControllerState`].
    pub controller: ControllerState,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`ControllerPhysicsBundle`]
    #[bundle]
    pub physics: ControllerPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`ComputedVisibility`]
    pub computed_visibility: ComputedVisibility,
}

impl Default for ControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            settings: ControllerSettings::character(),
            input: default(),
            physics: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

impl ControllerBundle {
    /// Construct this bundle with [`ControllerSettings::character()`]
    pub fn character() -> Self {
        Self {
            settings: ControllerSettings::character(),
            ..default()
        }
    }

    /// Construct this bundle with [`ControllerSettings::starship()`]
    pub fn starship() -> Self {
        Self {
            settings: ControllerSettings::starship(),
            ..default()
        }
    }
}
