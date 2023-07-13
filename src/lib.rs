/*
#![deny(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
 */
#![doc = include_str!("../README.md")]

mod bundles;
mod controller;
mod physics;
mod plugins;
mod spring;

#[cfg(feature = "rapier")]
mod rapier;

pub use {
    bundles::ControllerBundle, controller::*, physics::ControllerPhysicsBundle,
    plugins::WanderlustPlugin, rapier::*, spring::Spring,
};
