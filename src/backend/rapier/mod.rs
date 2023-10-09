//use crate::{controller::*, physics::*};
use crate::*;
use bevy::prelude::*;

#[cfg(feature = "rapier2d")]
pub use bevy_rapier2d as rapier;
#[cfg(feature = "rapier3d")]
pub use bevy_rapier3d as rapier;

use rapier::prelude::*;

pub fn backend_label() -> PhysicsSet {
    PhysicsSet::SyncBackend
}

mod bundle;
pub use bundle::RapierPhysicsBundle;
mod mass;
pub use mass::get_mass_from_backend;
mod velocity;
pub use velocity::get_velocity_from_backend;
mod query;
mod plugin;
pub use plugin::WanderlustRapierPlugin;

use rapier::prelude::Collider;

/// Apply forces to the controller to make it float, move, jump, etc.
pub fn apply_forces(
    ctx: Res<RapierContext>,
    mut forces: Query<(&mut ExternalImpulse, &ControllerForce)>,
) {
    let dt = ctx.integration_parameters.dt;
    for (mut impulse, force) in &mut forces {
        impulse.impulse += force.linear * dt;
        impulse.torque_impulse += force.angular * dt;
    }
}

/// Apply the opposing ground force to the entity we are pushing off of to float.
pub fn apply_ground_forces(
    ctx: Res<RapierContext>,
    mut impulses: Query<&mut ExternalImpulse>,
    ground_forces: Query<(&GroundForce, &ViableGroundCast)>,
) {
    let dt = ctx.integration_parameters.dt;
    for (force, cast) in &ground_forces {
        if let Some(ground) = cast.current() {
            if let Some(ground_body) = ctx.collider_parent(ground.entity) {
                if let Ok(mut impulse) = impulses.get_mut(ground_body) {
                    impulse.impulse += force.linear * dt;
                    impulse.torque_impulse += force.angular * dt;
                }
            }
        }
    }
}

pub fn update_delta_time(mut physics_dt: ResMut<PhysicsDeltaTime>, ctx: Res<RapierContext>) {
    physics_dt.0 = ctx.integration_parameters.dt;
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// Alternatively, if one only wants to disable the system, use [`WanderlustPhysicsTweaks`](WanderlustPhysicsTweaks).*
///
/// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
pub fn setup_physics_context(mut ctx: ResMut<RapierContext>) {
    let params = &mut ctx.integration_parameters;
    // This prevents any noticeable jitter when running facefirst into a wall.
    params.erp = 0.99;
    // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
    params.max_velocity_iterations = 16;
    // TODO: Fix jitter that occurs when running facefirst into a normal corner.
}
