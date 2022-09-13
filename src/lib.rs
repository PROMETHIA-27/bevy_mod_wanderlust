#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod backends;
mod bundles;
mod components;
mod plugins;
mod presets;
mod resources;
mod systems;

pub use self::{
    backends::PhysicsBackend,
    bundles::{CharacterControllerBundle, StarshipControllerBundle},
    components::{ControllerInput, ControllerSettings, ControllerState},
    plugins::WanderlustPlugin,
    presets::{CharacterControllerPreset, StarshipControllerPreset},
    resources::WanderlustPhysicsTweaks,
    systems::movement,
};
