#![deny(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![doc = include_str!("../README.md")]

mod bundles;
mod components;
mod plugins;
mod resources;
mod spring;
mod systems;

pub use {
    bundles::{ControllerBundle, ControllerPhysicsBundle},
    components::{ControllerInput, ControllerSettings, ControllerState},
    plugins::WanderlustPlugin,
    resources::WanderlustPhysicsTweaks,
    spring::Spring,
    systems::{movement, setup_physics_context},
};
