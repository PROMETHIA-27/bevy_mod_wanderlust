use std::ops::Neg;

use crate::components::{
    ContinuousMovement, ControllerInput, ControllerSettings, ControllerState, CoyoteTime,
    ExtraJumps, FinalMotion, Float, FloatImpulse, Gravity, GroundCast, GroundCaster, Grounded,
    Jumping, Mass, UprightImpulse, UprightSpring, Velocity,
};
use crate::WanderlustPhysicsTweaks;
use bevy::ecs::system::SystemParam;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(SystemParam)]
pub struct MovementParams<'w, 's> {
    bodies: Query<
        'w,
        's,
        (
            Entity,
            &'static mut ControllerState,
            &'static ControllerSettings,
            &'static mut ControllerInput,
            &'static mut GroundCast,
        ),
    >,
    velocities: Query<'w, 's, &'static Velocity>,
    globals: Query<'w, 's, &'static GlobalTransform>,
    masses: Query<'w, 's, &'static ReadMassProperties>,
    impulses: Query<'w, 's, &'static mut ExternalImpulse>,
    ctx: ResMut<'w, RapierContext>,
}

/* Setup phase */
/// Cache the up vector as the normalized negation of acceleration due to gravity.
pub fn set_up_vector(query: Query<&mut Gravity>) {
    for gravity in &mut query {
        gravity.up_vector = gravity.acceleration.neg().normalize_or_zero();
    }
}

/* Action phase */

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// This system is useful for cases such as running on a fixed timestep.*
///
/// The system that controls movement logic.
pub fn movement(
    params: MovementParams,
    mut ground_casts: Local<Vec<(Entity, Toi)>>,

    #[cfg(feature = "debug_lines")] mut gizmos: Gizmos,
) {
    let MovementParams {
        mut bodies,
        velocities,
        globals,
        masses,
        mut impulses,
        ctx,
    } = params;

    for (entity, mut controller, settings, input, ground_cast) in bodies.iter_mut() {
        // Things we do per iter:
        // - ground cast to find certain info
        // - if we hit something, reset ground timer
        // - determine groundedness
        // - determine jumps/coyote time
        // - determine contribution from gravity
        // - get velocity
        // - determine float_spring force
        // - calculate continuous movement input contribution
        // - determine jump contribution
        // - calculate upright force
        // - apply forces
        // ----- Finished line
        // - apply opposite forces to things being stood on

        // Opposite force to whatever we were touching

        controller.jump_pressed_last_frame = input.jumping;
        controller.is_grounded = grounded;
    }
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// Alternatively, if one only wants to disable the system, use [`WanderlustPhysicsTweaks`](WanderlustPhysicsTweaks).*
///
/// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
pub fn setup_physics_context(
    mut ctx: ResMut<RapierContext>,
    should_change: Option<Res<WanderlustPhysicsTweaks>>,
) {
    if should_change.map(|s| s.should_do_tweaks()).unwrap_or(true) {
        let params = &mut ctx.integration_parameters;
        // This prevents any noticeable jitter when running facefirst into a wall.
        params.erp = 0.99;
        // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
        params.max_velocity_iterations = 16;
        // TODO: Fix jitter that occurs when running facefirst into a normal corner.
    }
}