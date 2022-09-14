use bevy::{math::*, prelude::*};

use crate::backends::BackendCastableShape;
use crate::{CharacterControllerPreset, StarshipControllerPreset};

/// The character controller's state.
/// This is the component responsible for adding controls to an entity.
/// Requires [`ControllerSettings`], [`ControllerInput`], [`GlobalTransform`], and [`ExternalImpulse`](bevy_rapier3d::prelude::ExternalImpulse) to work.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ControllerState {
    /// Every frame, as part of input -> movement translation, a goal velocity is calculated.
    /// The goal velocity represents the input after being directly translated to a desired final motion.
    /// This field represents the goal velocity that was calculated last frame.
    pub last_goal_velocity: Vec3,
    /// A timer to track how long to skip the ground check for (see [`jump_skip_ground_check_duration`](ControllerSettings::jump_skip_ground_check_duration)).
    pub skip_ground_check_timer: f32,
    /// A timer to track how long to jump for.
    pub jump_timer: f32,
    /// Was [`ControllerInput::jumping`] pressed last frame.
    pub jump_pressed_last_frame: bool,
    /// A timer to track coyote time. See [`coyote_time_duration`](ControllerSettings::coyote_time_duration)
    pub coyote_timer: f32,
    /// A timer to track jump buffering. See [`jump_buffer_duration`](ControllerSettings::jump_buffer_duration)
    pub jump_buffer_timer: f32,
    /// How many extra jumps are remaining
    pub remaining_jumps: u32,
}

/// The settings of a character controller. See each individual field for more description.
///
/// The [`Default::default()`] of this type is not well configured; it is not a good reference for any character controller, and will not do much.
/// See bundles like [`CharacterControllerBundle`](super::bundles::CharacterControllerBundle) for well-config
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ControllerSettings<C: BackendCastableShape> {
    /// How quickly to interpolate from `last_goal_velocity` to the new `input_goal_velocity`.
    /// In other words, how quickly to go from "not moving" to "moving at max speed".
    pub acceleration: f32,
    /// The length of the calculated `input_goal_velocity`.
    /// In other words, the speed to attempt to reach if a movement input (such as forwards) is fully saturated.
    ///
    /// Keys are generally either not saturated or fully saturated, while analog controls like a joystick can be partially saturated (half tilt).
    pub max_speed: f32,
    /// The maximum amount of force that can be applied to fulfill [`acceleration`](ControllerSettings::acceleration).
    pub max_acceleration_force: f32,
    /// The direction to jump, which is also the direction that gravity is opposite to.
    pub up_vector: Vec3,
    /// The strength of gravity.
    pub gravity: f32,
    /// The maximum angle that the ground can be, in radians, before it is no longer considered suitable for being "grounded" on.
    ///
    /// For example, if this is set to `π/4` (45 degrees), then a player standing on a slope steeper than 45 degrees will slip and fall, and will not have
    /// their jump refreshed by landing on that surface.
    pub max_ground_angle: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much lower than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub min_float_offset: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much higher than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub max_float_offset: f32,
    /// The amount of force to apply on the first frame when a jump begins.
    pub jump_initial_force: f32,
    /// The amount of force to continuously apply every second during a jump.
    pub jump_force: f32,
    /// The amount of force to apply downwards when the jump control is released prior to a jump expiring.
    /// This allows analog jumping by cutting the jump short when the control is released.
    pub jump_stop_force: f32,
    /// How long a jump can last.
    pub jump_time: f32,
    /// A function taking the current progress of a jump, from 0.0 to 1.0, with 0.0 indicating a jump has just begun and 1.0 indicating the jump has ended,
    /// which returns a modifier (usually from 0.0 to 1.0, but not necessarily) to multiply [`jump_force`](ControllerSettings::jump_force) by.
    #[reflect(ignore)]
    pub jump_decay_function: fn(f32) -> f32,
    /// How long to skip ground checks after jumping. Usually this should be set just high enough that the character is out of range of the ground
    /// just before the timer elapses.
    pub jump_skip_ground_check_duration: f32,
    /// How many extra times the character can jump after leaving the ground. 0 is normal, 1 corresponds to double jump, etc.
    pub extra_jumps: u32,
    /// How long should the character still be able to jump after leaving the ground, in seconds.
    /// For example, if this is set to 0.5, the player can fall off a ledge and then jump if they do so within 0.5 seconds of leaving the ledge.
    pub coyote_time_duration: f32,
    /// If the jump input is pressed before landing, how long will the jump be buffered for?
    /// In other words, if this is 0.5, the character can input jump up to 0.5 seconds before landing and the jump will occur when they land.
    pub jump_buffer_duration: f32,
    /// Scales movement force. This is useful to ensure movement does not affect vertical velocity (by setting it to e.g. `Vec3(1.0, 0.0, 1.0)`).
    pub force_scale: Vec3,
    /// How long of a ray to cast to detect the ground. Setting this unnecessarily high will permanently count the player as grounded,
    /// and too low will allow the player to slip and become disconnected from the ground easily.
    pub float_cast_length: f32,
    /// An offset to start the ground check from, relative to the character's origin.
    pub float_cast_origin: Vec3,
    /// What shape of ray to cast. See
    /// [`PhysicsBackend::cast_shape`](crate::PhysicsBackend::cast_shape).
    #[reflect(ignore)]
    pub float_cast_collider: C,
    /// How far to attempt to float away from the ground.
    pub float_distance: f32,
    /// How strongly to float away from the ground.
    pub float_strength: f32,
    /// How strongly to dampen floating away from the ground, to prevent jittering/oscillating float movement.
    pub float_dampen: f32,
    /// How strongly to attempt to stay upright. Alternatively, see Rapier's `LockedAxes` to lock
    /// rotation entirely.
    pub upright_spring_strength: f32,
    /// How strongly to dampen staying upright. Prevents jittering/oscillating upright movement.
    pub upright_spring_damping: f32,
}

impl<C: BackendCastableShape> ControllerSettings<C> {
    /// See [`CharacterControllerPreset`].
    pub fn character() -> Self {
        CharacterControllerPreset.into()
    }

    /// See [`StarshipControllerPreset`].
    pub fn starship() -> Self {
        StarshipControllerPreset.into()
    }
}

impl<C: BackendCastableShape> Default for ControllerSettings<C> {
    fn default() -> Self {
        Self {
            acceleration: default(),
            max_speed: default(),
            max_acceleration_force: default(),
            up_vector: default(),
            gravity: default(),
            max_ground_angle: default(),
            min_float_offset: default(),
            max_float_offset: default(),
            jump_initial_force: default(),
            jump_force: default(),
            jump_stop_force: default(),
            jump_time: 1.0,
            jump_decay_function: |_| 1.0,
            jump_skip_ground_check_duration: default(),
            extra_jumps: default(),
            coyote_time_duration: default(),
            jump_buffer_duration: default(),
            force_scale: default(),
            float_cast_length: default(),
            float_cast_origin: default(),
            float_cast_collider: C::ball(1.0),
            float_distance: default(),
            float_strength: default(),
            float_dampen: default(),
            upright_spring_strength: default(),
            upright_spring_damping: default(),
        }
    }
}

/// This is the interface for applying input to the character controller.
/// See each field for more information.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ControllerInput {
    /// This field represents movement in 3D space.
    /// The majority of games will map this to WASD/Analog joystick in 2D space along the ground.
    /// To ensure movement does not affect the Y axis, set [`ControllerSettings::force_scale`] to `Vec3(1.0, 0.0, 1.0)`.
    /// This field will be normalized when read by the movement system.
    pub movement: Vec3,
    /// This field represents if the jump control is currently pressed.
    pub jumping: bool,
    /// Allows supplying a custom force to the controller to be applied next frame,
    /// which is necessary because the controller monopolizes and controls the `ExternalImpulse`
    /// which rapier uses to apply impulse forces to a rigidbody.
    /// Will be reset to 0 after being applied.
    pub custom_impulse: Vec3,
    /// Allows supplying a custom torque to the controller to be applied next frame,
    /// which is necessary because the controller monopolizes and controls the `ExternalImpulse`
    /// which rapier uses to apply impulse forces to a rigidbody.
    /// Will be reset to 0 after being applied.
    pub custom_torque: Vec3,
}
