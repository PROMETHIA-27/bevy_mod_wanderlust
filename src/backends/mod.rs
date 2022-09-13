//! This module contains the [PhysicsBackend] trait, and its submodules implement it for various
//! physics backends.

use bevy::prelude::*;

#[cfg(feature = "rapier2d")]
mod rapier2d;
#[cfg(feature = "rapier2d")]
pub use rapier2d::{Rapier2dBackend, Rapier2dControllerPhysicsBundle};

#[cfg(feature = "rapier3d")]
mod rapier3d;
#[cfg(feature = "rapier3d")]
pub use rapier3d::{Rapier3dBackend, Rapier3dControllerPhysicsBundle};

/// The trait to implement in order to support a backend.
///
/// This trait serves as the generic argument for [`WanderlustPlugin`](crate::WanderlustPlugin) and
/// for [`CharacterControllerBundle`](crate::WanderlustPlugin).
pub trait PhysicsBackend: 'static + Send + Sync {
    /// A bundle of components to add to the player controlled entity as part of
    /// [`CharacterControllerBundle`](crate::CharacterControllerBundle).
    ///
    /// Should `impl` [Default] with sensible defaults, but the values of the components should
    /// also be overridable by the user.
    type ControllerPhysicsBundle: Bundle + Default;

    /// A component used by the backend to apply impulses.
    type ExternalImpulse: Component;

    /// A component used by the backend to represent the linear and angular velocity of the rigid
    /// body.
    type Velocity: Component;

    /// A resource that can be used to query the physics engine.
    type PhysicsContext: Send + Sync;

    /// A component used by the backend to represent a shape that can be cast to determine the
    /// distance to the ground.
    type CastableShape: BackendCastableShape;

    /// Generate a setup system that can prepare the backend to be used with Wanderlust.
    ///
    /// *Note: Most users will not need to use this directly. Use
    /// [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead. Alternatively, if one
    /// only wants to disable the system, use
    /// [`WanderlustPhysicsTweaks`](crate::WanderlustPhysicsTweaks).*
    fn generate_setup_system_set() -> SystemSet;

    /// Apply impulses to the component that represents impulses.
    fn apply_impulses(body: &mut Self::ExternalImpulse, impulse: Vec3, torque_impulse: Vec3);

    /// Checks if an entity is in contact with any other collider.
    fn entity_has_contacts(ctx: &Self::PhysicsContext, entity: Entity) -> bool;

    /// Cast a shape to determine the distance to the ground.
    fn cast_shape(
        ctx: &Self::PhysicsContext,
        transofrm: &GlobalTransform,
        settings: &crate::ControllerSettings<Self::CastableShape>,
        entity: Entity,
    ) -> Option<(Entity, ToiProxy)>;

    /// Extract the linear part from the velocity component.
    fn extract_linvel(velocity: &Self::Velocity) -> Vec3;

    /// Extract the angular part from the velocity component.
    fn extract_angvel(velocity: &Self::Velocity) -> Vec3;
}

/// The result of a time-of-impact (TOI) computation.
///
/// Different backends will have different versions of this, so the backend needs to translate them
/// to this struct. It only contains the fields Wanderlust uses.
#[derive(Clone, Copy)]
pub struct ToiProxy {
    /// The time at which the objects touch.
    pub toi: f32,
    /// The local-space outward normal on the first shape at the time of impact.
    pub normal1: Vec3,
    /// The way the time-of-impact computation algorithm terminated.
    pub status: TOIStatusProxy,
}

/// The status of the time-of-impact computation algorithm.
///
/// Different backends will have different versions of this, so the backend needs to translate them
/// to this enum.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TOIStatusProxy {
    /// The TOI algorithm ran out of iterations before achieving convergence.
    OutOfIterations,
    /// The TOI algorithm converged successfully.
    Converged,
    /// Something went wrong during the TOI computation, likely due to numerical instabilities.
    Failed,
    /// The two shape already overlap at the time 0.
    Penetrating,
}

/// A trait for generic usage of the castable shape defined by [PhysicsBackend::CastableShape].
pub trait BackendCastableShape: 'static + Send + Sync {
    /// Create a castable shape in the shape of a ball.
    fn ball(radius: f32) -> Self;
}
