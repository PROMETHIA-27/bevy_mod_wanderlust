use crate::{Controller, ControllerInput, ControllerPhysicsBundle, RapierPhysicsBundle};

use bevy::prelude::*;

/// The recommended bundle for creating a character controller. Includes the necessary components for a character controller
/// as well as many physics-related components that can be used to tweak the behavior of the controller. Try using the
/// [`Self::builder()`] method to construct the bundle!
#[derive(Bundle)]
pub struct ControllerBundle {
    /// See [`Controller`].
    pub controller: Controller,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`PhysicsBundle`]
    pub physics: ControllerPhysicsBundle,
    #[cfg(feature = "rapier")]
    /// See [`RapierPhysicsBundle`]
    pub rapier_physics: RapierPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`IinheritedVisibility`]
    pub inherited_visibility: InheritedVisibility,
}

impl Default for ControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            input: default(),
            physics: default(),
            #[cfg(feature = "rapier")]
            rapier_physics: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
        }
    }
}

impl ControllerBundle {
    /// Construct this bundle with [`ControllerSettings::character()`]
    pub fn character() -> Self {
        Self { ..default() }
    }

    /// Construct this bundle with [`ControllerSettings::starship()`]
    pub fn starship() -> Self {
        Self { ..default() }
    }
}
