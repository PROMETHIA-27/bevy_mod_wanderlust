use crate::{ControllerInput, ControllerSettings, ControllerState, PhysicsBackend};

use bevy::prelude::*;

/// The recommended bundle for creating a basic, walking character controller. Includes the necessary components for a character controller
/// as well as many physics-related components that can be used to tweak the behavior of the controller, with reasonable default
/// values.
#[derive(Bundle)]
pub struct CharacterControllerBundle<B: PhysicsBackend> {
    /// See [`CharacterController`].
    pub controller: ControllerState,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings<B::CastableShape>,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`PhysicsBackend::ControllerPhysicsBundle`]
    #[bundle]
    pub physics: B::ControllerPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`ComputedVisibility`]
    pub computed_visibility: ComputedVisibility,
}

impl<B: PhysicsBackend> Default for CharacterControllerBundle<B> {
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

/// A flying character controller with spaceship-like controls.
#[derive(Bundle)]
pub struct StarshipControllerBundle<B: PhysicsBackend> {
    /// See [`ControllerState`].
    pub controller: ControllerState,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings<B::CastableShape>,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`PhysicsBackend::ControllerPhysicsBundle`]
    #[bundle]
    pub physics: B::ControllerPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`ComputedVisibility`]
    pub computed_visibility: ComputedVisibility,
}

impl<B: PhysicsBackend> Default for StarshipControllerBundle<B> {
    fn default() -> Self {
        Self {
            controller: default(),
            settings: ControllerSettings::starship(),
            input: default(),
            physics: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}
