#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod bundles;
mod components;
mod plugins;
mod systems;

pub use self::{
    bundles::CharacterControllerBundle,
    components::{CharacterController, ControllerSettings},
    plugins::WanderlustPlugin,
};
