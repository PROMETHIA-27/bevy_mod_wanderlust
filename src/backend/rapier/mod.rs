//use crate::{controller::*, physics::*};
use bevy::prelude::*;

#[cfg(feature = "rapier2d")]
pub use bevy_rapier2d as rapier;
#[cfg(feature = "rapier3d")]
pub use bevy_rapier3d as rapier;

mod bundle;
pub use bundle::RapierPhysicsBundle;
mod mass;
pub use mass::*;
mod velocity;
pub use velocity::*;
mod query;
pub use query::*;

pub use rapier::prelude::Collider;

/*
/// Apply forces to the controller to make it float, move, jump, etc.
pub fn apply_forces(
    mut forces: Query<(&mut ExternalImpulse, &ControllerForce)>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (mut impulse, force) in &mut forces {
        impulse.impulse += force.linear * dt;
        impulse.torque_impulse += force.angular * dt;
    }
}

/// Apply the opposing ground force to the entity we are pushing off of to float.
pub fn apply_ground_forces(
    mut impulses: Query<&mut ExternalImpulse>,
    ground_forces: Query<(&GroundForce, &GroundCast)>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (force, cast) in &ground_forces {
        if let GroundCast::Touching(ground) = cast {
            if let Some(ground_body) = ctx.collider_parent(ground.entity) {
                if let Ok(mut impulse) = impulses.get_mut(ground_body) {
                    impulse.impulse += force.linear * dt;
                    impulse.torque_impulse += force.angular * dt;
                }
            }
        }
    }
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
 */