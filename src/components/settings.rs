use bevy::ecs::reflect::ReflectComponent;
use bevy::math::vec3;
use bevy::prelude::{default, warn, Component, Entity, Vec3};
use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::Collider;

use crate::Spring;

/// The settings of a character controller. See each individual field for more description.
///
/// The [`Default::default()`] of this type is not well configured; it is not a good reference for any character controller, and will not do much.
/// See bundles like [`CharacterControllerBundle`](super::bundles::CharacterControllerBundle) for well-config
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ControllerSettings {
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
    /// The direction to face towards, or `None` to not rotate to face any direction. Must be perpendicular to the up vector and normalized.
    pub forward_vector: Option<Vec3>,
    /// The strength of gravity.
    pub gravity: f32,
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
    pub jump_decay_function: Option<fn(f32) -> f32>,
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
    /// How far to attempt to float away from the ground.
    pub float_distance: f32,
    /// How strongly to float away from the ground.
    pub float_spring: Spring,
    /// How strongly to force the character upright/avoid overshooting. Alternatively, see [`LockedAxes`] to lock rotation entirely.
    pub upright_spring: Spring,
    /// Scaling factor for the impulse applied to the ground to keep the character moving/off the ground.
    pub opposing_impulse_scale: f32,
    /// Scaling factor for the movement impulse applied to the ground.
    /// Setting this to 0.0 would make it so things don't "slip" out from the characters feet.
    pub opposing_movement_impulse_scale: f32,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            acceleration: default(),
            max_speed: default(),
            max_acceleration_force: default(),
            up_vector: default(),
            forward_vector: default(),
            gravity: default(),
            max_ground_angle: default(),
            min_float_offset: default(),
            max_float_offset: default(),
            jump_initial_force: default(),
            jump_force: default(),
            jump_stop_force: default(),
            jump_time: 1.0,
            jump_decay_function: None,
            jump_skip_ground_check_duration: default(),
            skip_ground_check_override: default(),
            extra_jumps: default(),
            coyote_time_duration: default(),
            jump_buffer_duration: default(),
            force_scale: default(),
            float_cast_length: default(),
            float_cast_origin: default(),
            float_cast_collider: Collider::ball(1.0),
            float_distance: default(),
            float_spring: default(),
            upright_spring: default(),
            exclude_from_ground: default(),
            opposing_impulse_scale: 1.0,
            opposing_movement_impulse_scale: 1.0,
        }
    }
}

impl ControllerSettings {
    /// A basic preset for a standard, walking character controller. Works for most first and third person games.
    pub fn character() -> Self {
        ControllerSettings {
            acceleration: 50.0,
            max_speed: 10.0,
            max_acceleration_force: 10.0,
            up_vector: Vec3::Y,
            //gravity: -9.8,
            gravity: -20.0,
            max_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
            min_float_offset: -0.3,
            max_float_offset: 0.05,
            jump_time: 0.5,
            jump_initial_force: 15.0,
            jump_stop_force: 0.3,
            jump_decay_function: Some(|x| (1.0 - x).sqrt()),
            jump_skip_ground_check_duration: 0.5,
            coyote_time_duration: 0.16,
            jump_buffer_duration: 0.16,
            force_scale: vec3(1.0, 0.0, 1.0),
            float_cast_length: 1.0,
            float_cast_collider: Collider::ball(0.45),
            float_distance: 0.55,
            float_spring: Spring {
                strength: 100.0,
                damping: 0.8,
            },
            upright_spring: Spring {
                strength: 10.0,
                damping: 0.5,
            },
            ..default()
        }
    }

    /// A sample controller preset for a spaceship which can fly in any direction.
    pub fn starship() -> Self {
        ControllerSettings {
            acceleration: 0.3,
            max_speed: 100.0,
            max_acceleration_force: 10.0,
            up_vector: Vec3::Y,
            force_scale: vec3(1.0, 1.0, 1.0),
            upright_spring: Spring {
                strength: 0.0,
                damping: 0.0,
            },
            ..default()
        }
    }

    /// Validate that assumptions made in the settings are correct.
    pub fn valid(&self) -> bool {
        let mut valid = true;
        if !self.up_vector.is_normalized() {
            warn!("controller up vector is not normalized");
            valid = false;
        }

        valid
    }
}
