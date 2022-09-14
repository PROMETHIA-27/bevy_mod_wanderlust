use crate::backends::BackendCastableShape;
use crate::ControllerSettings;
use bevy::math::vec3;
use bevy::prelude::*;

/// A basic preset for a standard, walking character controller. Works for most first and third person games.
pub struct CharacterControllerPreset;

impl<C: BackendCastableShape> From<CharacterControllerPreset> for ControllerSettings<C> {
    fn from(_: CharacterControllerPreset) -> ControllerSettings<C> {
        ControllerSettings {
            acceleration: 50.0,
            max_speed: 10.0,
            max_acceleration_force: 10.0,
            up_vector: Vec3::Y,
            gravity: 25.0,
            max_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
            min_float_offset: -0.3,
            max_float_offset: 0.05,
            jump_time: 0.5,
            jump_initial_force: 15.0,
            jump_stop_force: 0.3,
            jump_decay_function: |x| (1.0 - x).sqrt(),
            jump_skip_ground_check_duration: 0.5,
            coyote_time_duration: 0.16,
            jump_buffer_duration: 0.16,
            force_scale: vec3(1.0, 0.0, 1.0),
            float_cast_length: 1.0,
            float_cast_collider: C::ball(0.45),
            float_distance: 0.55,
            float_strength: 10.0,
            float_dampen: 0.5,
            upright_spring_strength: 100.0,
            upright_spring_damping: 10.0,
            ..default()
        }
    }
}

/// A sample controller preset for a spaceship which can fly in any direction.
pub struct StarshipControllerPreset;

impl<C: BackendCastableShape> From<StarshipControllerPreset> for ControllerSettings<C> {
    fn from(_: StarshipControllerPreset) -> ControllerSettings<C> {
        ControllerSettings {
            acceleration: 0.3,
            max_speed: 100.0,
            max_acceleration_force: 10.0,
            up_vector: Vec3::Y,
            force_scale: vec3(1.0, 1.0, 1.0),
            ..default()
        }
    }
}
