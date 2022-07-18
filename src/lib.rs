#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod bundles;
mod components;
mod plugins;
mod presets;
mod resources;
mod systems;

pub use self::{
    bundles::{CharacterControllerBundle, StarshipControllerBundle},
    components::{ControllerInput, ControllerSettings, ControllerState},
    plugins::WanderlustPlugin,
    presets::{CharacterControllerPreset, StarshipControllerPreset},
    resources::WanderlustPhysicsTweaks,
};
