use crate::controller::*;
/// Movements applied via inputs.
///
/// This includes directional movement and jumping.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Settings used to determine movement impulses on this controller.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Movement {
    /// How fast to get to the max speed.
    pub acceleration: f32,
    /// How fast our controller will move.
    pub max_speed: f32,
    /// Scales movement force. This is useful to ensure movement does not
    /// affect vertical velocity (by setting it to e.g. `Vec3(1.0, 0.0, 1.0)`).
    pub force_scale: Vec3,
}

/// Calculated impulse for moving the character.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MovementForce {
    /// Linear impulse to apply to move the character.
    pub linear: Vec3,
}

pub fn movement_force(
    mut query: Query<(
        &mut MovementForce,
        &Movement,
        &ControllerInput,
        &GroundCast,
        &ControllerVelocity,
    )>,
) {
    for (mut force, movement, input, ground, velocity) in &mut query {
        force.linear = {
            let ground_velocity = ground
                .cast
                .map(|(_, _, vel)| vel.linvel)
                .unwrap_or_default();

            let dir = input.movement.clamp_length_max(1.0);
            let goal = dir * movement.max_speed;

            let relative_velocity = velocity.linear - ground_velocity;
            let velocity_displacement = goal - relative_velocity;
            velocity_displacement.clamp_length_max(movement.acceleration)
        };
    }
}

#[derive(Default, Reflect)]
#[reflect(Default)]
pub struct ExtraJumps {
    /// How many extra times the character can jump after leaving the ground. 0 is normal, 1 corresponds to double jump, etc.
    pub extra: u32,
    /// How many extra jumps are remaining
    pub remaining: u32,
}

#[derive(Default, Reflect)]
#[reflect(Default)]
pub struct CoyoteTime {
    /// How long should the character still be able to jump after leaving the ground, in seconds.
    /// For example, if this is set to 0.5, the player can fall off a ledge and then jump if they do so within 0.5 seconds of leaving the ledge.
    pub duration: f32,
    /// A timer to track coyote time. See [`coyote_time_duration`](Self::coyote_time_duration)
    pub timer: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Jumping {
    /// Was [`ControllerInput::jumping`] true last frame.
    pub pressed_last_frame: bool,
    /// A timer to track how long to jump for.
    pub timer: f32,
    /// A timer to track jump buffering. See [`jump_buffer_duration`](ControllerSettings::jump_buffer_duration)
    pub buffer_timer: f32,
    /// The amount of force to apply on the first frame when a jump begins.
    pub initial_force: f32,
    /// The amount of force to continuously apply every second during a jump.
    pub force: f32,
    /// The amount of force to apply downwards when the jump control is released prior to a jump expiring.
    /// This allows analog jumping by cutting the jump short when the control is released.
    pub stop_force: f32,
    /// How long a jump can last.
    pub time: f32,
    /// If the jump input is pressed before landing, how long will the jump be buffered for?
    /// In other words, if this is 0.5, the character can input jump up to 0.5 seconds before landing and the jump will occur when they land.
    pub buffer_duration: f32,
    /// A function taking the current progress of a jump, from 0.0 to 1.0, with 0.0 indicating a jump has just begun and 1.0 indicating the jump has ended,
    /// which returns a modifier (usually from 0.0 to 1.0, but not necessarily) to multiply [`jump_force`](ControllerSettings::jump_force) by.
    #[reflect(ignore)]
    pub decay_function: Option<fn(f32) -> f32>,
    /// How long to skip ground checks after jumping. Usually this should be set just high enough that the character is out of range of the ground
    /// just before the timer elapses.
    pub skip_ground_check_duration: f32,

    pub coyote_time: CoyoteTime,
    pub extra_jumps: ExtraJumps,
}

/// Calculated force for character jumping.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct JumpForce {
    /// Linear impulse to apply to push the character up.
    pub linear: Vec3,
}

pub fn jump_force(
    mut query: Query<(
        &mut JumpForce,
        &mut Jumping,
        &mut GroundCaster,
        &ControllerInput,
        &Grounded,
        &Gravity,
        &ControllerVelocity,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;

    for (mut force, mut jumping, mut groundcaster, input, grounded, gravity, velocity) in &mut query
    {
        let grounded = **grounded;
        let just_jumped = input.jumping && !jumping.pressed_last_frame;
        if grounded {
            jumping.extra_jumps.remaining = jumping.extra_jumps.extra;
            jumping.coyote_time.timer = jumping.coyote_time.duration;
        } else {
            jumping.coyote_time.timer = (jumping.coyote_time.timer - dt).max(0.0);

            if just_jumped {
                jumping.buffer_timer = jumping.buffer_duration;
            } else {
                jumping.buffer_timer = (jumping.buffer_timer - dt).max(0.0);
            }
        }

        // Calculate jump force
        let mut jump = if jumping.timer > 0.0 && !grounded {
            if !input.jumping {
                jumping.timer = 0.0;
                velocity.linear.project_onto(gravity.up_vector) * -jumping.stop_force
            } else {
                jumping.timer = (jumping.timer - dt).max(0.0);

                jumping.force
                    * gravity.up_vector
                    * jumping
                        .decay_function
                        .map(|f| (f)((jumping.time - jumping.timer) / jumping.time))
                        .unwrap_or(1.0)
            }
        } else {
            Vec3::ZERO
        };

        // Trigger a jump
        let coyote_timer = jumping.coyote_time.timer;
        let remaining_jumps = jumping.extra_jumps.remaining;
        if (just_jumped || jumping.buffer_timer > 0.0)
            && (grounded || coyote_timer > 0.0 || remaining_jumps > 0)
        {
            if !grounded && coyote_timer == 0.0 {
                jumping.extra_jumps.remaining -= 1;
            }

            jumping.buffer_timer = 0.0;
            jumping.timer = jumping.time;
            groundcaster.skip_ground_check_timer = jumping.skip_ground_check_duration;
            // Negating the current velocity increases consistency for falling jumps,
            // and prevents stacking jumps to reach high upwards velocities
            force.linear = velocity.linear * gravity.up_vector * -1.0;
            force.linear += jumping.initial_force * gravity.up_vector;
        }
    }
}
