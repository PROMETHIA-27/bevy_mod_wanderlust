#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod bundles;
mod components;
mod plugins;
mod resources;
mod systems;

pub use self::{
    bundles::CharacterControllerBundle,
    components::{CharacterController, ControllerInput, ControllerSettings},
    plugins::WanderlustPlugin,
    resources::WanderlustPhysicsTweaks,
};
