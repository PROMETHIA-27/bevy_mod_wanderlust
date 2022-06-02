use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    pub settings: ControllerSettings,
    pub last_goal_velocity: Vec3,
    pub skip_ground_check_timer: f32,
}

#[derive(Reflect)]
pub struct ControllerSettings {
    pub acceleration: f32,
    pub max_speed: f32,
    pub max_acceleration_force: f32,
    pub up_vector: Vec3,
    pub gravity: f32,
    pub max_ground_angle: f32,
    pub jump_force: f32,
    pub jump_skip_ground_check_duration: f32,
    pub force_scale: Vec3,
    pub float_cast_length: f32,
    pub float_cast_origin: Vec3,
    #[reflect(ignore)]
    pub float_cast_collider: Collider,
    pub float_distance: f32,
    pub float_strength: f32,
    pub float_dampen: f32,
    pub upright_spring_strength: f32,
    pub upright_spring_damping: f32,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            acceleration: default(),
            max_speed: default(),
            max_acceleration_force: default(),
            up_vector: default(),
            gravity: default(),
            max_ground_angle: default(),
            jump_force: default(),
            jump_skip_ground_check_duration: default(),
            force_scale: default(),
            float_cast_length: default(),
            float_cast_origin: default(),
            float_cast_collider: Collider::ball(1.0),
            float_distance: default(),
            float_strength: default(),
            float_dampen: default(),
            upright_spring_strength: default(),
            upright_spring_damping: default(),
        }
    }
}
