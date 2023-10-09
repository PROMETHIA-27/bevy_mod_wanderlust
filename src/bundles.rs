use crate::{Controller, ControllerInput, ControllerPhysicsBundle};

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
    /// See [`BackendPhysicsBundle`].
    pub backend_physics: crate::backend::BackendPhysicsBundle,
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
            input: default(),
            physics: default(),
            backend_physics: default(),
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
        Self { ..default() }
    }

    /// Construct this bundle with [`ControllerSettings::starship()`]
    pub fn starship() -> Self {
        Self { ..default() }
    }
}
