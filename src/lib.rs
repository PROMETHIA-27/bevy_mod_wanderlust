#![deny(missing_docs)]
//! # Wanderlust
//! Wanderlust is a character controller addon. Inspired by [this excellent video](https://www.youtube.com/watch?v=qdskE8PJy6Q) and
//! my previous attempts at creating a character controller, it is implemented on top of [Rapier physics](https://rapier.rs/)
//! and highly customizable.
//!
//! ```rust,no_run
//! # use bevy::prelude::*;
//! use bevy_mod_wanderlust::WanderlustPlugin;
//!
//! App::new().add_plugins(DefaultPlugins).add_plugin(WanderlustPlugin).run()
//! ```
//! 
//! Wanderlust does not handle mouselook, as it's more-or-less trivial to implement compared to movement, and would add significant complexity to build in
//! as many projects will have vastly different requirements for mouselook. The `simple.rs` example includes an example mouselook implementation.

mod bundles;
mod components;
mod plugins;
mod systems;

pub use self::{
    bundles::CharacterControllerBundle,
    components::{CharacterController, ControllerSettings},
    plugins::WanderlustPlugin,
};
