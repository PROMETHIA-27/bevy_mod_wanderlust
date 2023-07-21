use crate::controller::*;
/// Movements applied via inputs.
///
/// This includes directional movement and jumping.
use bevy_rapier3d::prelude::*;

/// Settings used to determine movement impulses on this controller.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Movement {
    /// How fast to get to the max speed.
    pub acceleration: f32,
    /// Caps acceleration so we don't overshoot too hard.
    pub max_acceleration_force: f32,
    /// How fast our controller will move.
    pub max_speed: f32,
    /// Scales movement force. This is useful to ensure movement does not
    /// affect vertical velocity (by setting it to e.g. `Vec3(1.0, 0.0, 1.0)`).
    pub force_scale: Vec3,
    // /// Stick to the same position on the ground.
    // pub stick_to_ground: Vec3,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 50.0,
            max_speed: 10.0,
            force_scale: Vec3::ONE,
            max_acceleration_force: 10.0,
            //stick_to_ground: true,
        }
    }
}

/// Calculated impulse for moving the character.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MovementForce {
    /// Linear impulse to apply to move the character.
    pub linear: Vec3,
    /// Angular impulse to apply to move the character.
    pub angular: Vec3,
}

/// Calculates the movement forces for this controller.
pub fn movement_force(
    mut query: Query<(
        &mut MovementForce,
        &mut Movement,
        &Gravity,
        &ControllerInput,
        &GroundCast,
        &GroundCaster,
        &ControllerVelocity,
    )>,
) {
    for (mut force, movement, gravity, input, cast, ground_caster, velocity) in &mut query {
        force.linear = Vec3::ZERO;

        let Some(ground) = cast.last() else { continue };
        let ground_angle = ground.cast.normal.angle_between(gravity.up_vector);
        let slipping = (ground.cast.normal.length() > 0.0 && ground_angle > ground_caster.max_ground_angle) || ground.cast.normal.length() == 0.0;
        if slipping {
            if let GroundCast::Touching(ground) = cast {
                let mut slip_vector = ground.cast.normal.reject_from_normalized(gravity.up_vector);
                slip_vector = slip_vector.normalize_or_zero();
                force.linear += slip_vector * -gravity.acceleration;
                force.linear += gravity.up_vector * gravity.acceleration * 5.0;
            }
        } else {
            let input_dir = input.movement.clamp_length_max(1.0);
            let input_goal_vel = input_dir * movement.max_speed;
            let goal_vel = input_goal_vel;
            let current_vel = velocity.linear - ground.point_velocity.linvel;

            let displacement = (goal_vel - current_vel) * movement.force_scale;
            force.linear += (displacement * movement.acceleration)
                .clamp_length_max(movement.max_acceleration_force);
        }
    }
}

/// How long should the character be considered grounded even after leaving the ground.
#[derive(Reflect, Debug, Clone)]
#[reflect(Default)]
pub struct CoyoteTime {
    /// How long should the character still be able to jump after leaving the ground, in seconds.
    /// For example, if this is set to 0.5, the player can fall off a ledge and then jump if they do so within 0.5 seconds of leaving the ledge.
    pub duration: f32,
    /// A timer to track coyote time. See [`coyote_time_duration`](Self::coyote_time_duration)
    pub timer: f32,
}

impl Default for CoyoteTime {
    fn default() -> Self {
        Self {
            duration: 0.16,
            timer: 0.0,
        }
    }
}

/// How the controller's jumping should behave.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Jump {
    /// The amount of force to apply on the first frame when a jump begins.
    pub initial_force: f32,
    /// How long to wait before we can jump again.
    pub cooldown_duration: f32,
    /// Timer for tracking `cooldown_duration`.
    pub cooldown_timer: f32,

    /// Number of times we can jump before we have to touch the ground again.
    pub jumps: u32,

    /// Remaining before we have to touch the ground again.
    pub remaining_jumps: u32,

    /// Was [`ControllerInput::jumping`] true last frame.
    pub pressed_last_frame: bool,
    /*
    /// A timer to track how long to jump for.
    pub timer: f32,
    /// A timer to track jump buffering. See [`jump_buffer_duration`](ControllerSettings::jump_buffer_duration)
    pub buffer_timer: f32,
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
    /// How long should the character be considered grounded even after leaving the ground.
    pub coyote_time: CoyoteTime,
    */
}

impl Default for Jump {
    fn default() -> Self {
        Self {
            initial_force: 100.0,
            cooldown_duration: 0.5,
            cooldown_timer: 0.0,
            jumps: 1,
            remaining_jumps: 1,
            pressed_last_frame: false,
            /*
                       buffer_timer: default(),
                       timer: 0.0,
                       force: 500.0,
                       time: 0.5,
                       stop_force: 0.3,
                       skip_ground_check_duration: 0.5,
                       decay_function: Some(|x| (1.0 - x).sqrt()),
                       buffer_duration: 0.16,
                       coyote_time: default(),
                       extra_jumps: default(),
            */
        }
    }
}

/// Calculated force for controller jumping.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct JumpForce {
    /// Linear impulse to apply to push the character up.
    pub linear: Vec3,
}

/// Calculate the jump force for the controller.
pub fn jump_force(
    mut query: Query<(
        &mut JumpForce,
        &mut FloatForce,
        &mut GravityForce,
        &mut Jump,
        &ControllerInput,
        &Grounded,
        &Gravity,
        &ControllerVelocity,
    )>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;

    for (
        mut force,
        mut float_force,
        mut gravity_force,
        mut jumping,
        input,
        grounded,
        gravity,
        velocity,
    ) in &mut query
    {
        force.linear = Vec3::ZERO;

        let grounded = **grounded;
        let just_jumped = input.jumping && !jumping.pressed_last_frame;

        if jumping.cooldown_timer > 0.0 {
            jumping.cooldown_timer -= dt;
        } else {
            if grounded {
                jumping.remaining_jumps = jumping.jumps;
            }
        }

        let can_jump = just_jumped && jumping.cooldown_timer <= 0.0 && jumping.remaining_jumps > 0;
        if can_jump {
            // Negating the current velocity increases consistency for falling jumps,
            // and prevents stacking jumps to reach high upwards velocities
            let initial_jump_force = jumping.initial_force * gravity.up_vector;
            let negate_velocity =
                (-1.0 * gravity.up_vector * velocity.linear.dot(gravity.up_vector)) / dt;
            force.linear = negate_velocity + initial_jump_force;

            gravity_force.linear = Vec3::ZERO;
            float_force.linear = Vec3::ZERO;

            info!("jumping: {:?} {:?}", negate_velocity, initial_jump_force);

            jumping.remaining_jumps = jumping.remaining_jumps.saturating_sub(1);
            jumping.cooldown_timer = jumping.cooldown_duration;
        }

        jumping.pressed_last_frame = input.jumping;
        /*
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
               if jumping.timer > 0.0 && !grounded {
                   force.linear = if !input.jumping {
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
                   };
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
                   force.linear = velocity.linear * gravity.up_vector * -1.0;
                   force.linear += jumping.initial_force * gravity.up_vector;
               }

        */
    }
}
