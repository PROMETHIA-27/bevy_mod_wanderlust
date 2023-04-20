#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod bundles;
mod components;
mod plugins;
mod resources;
mod spring;
mod systems;

pub use {
    bundles::{CharacterControllerBundle, ControllerPhysicsBundle, StarshipControllerBundle},
    components::{ControllerInput, ControllerSettings, ControllerState},
    plugins::WanderlustPlugin,
    resources::WanderlustPhysicsTweaks,
    spring::Spring,
    systems::{movement, setup_physics_context},
};
