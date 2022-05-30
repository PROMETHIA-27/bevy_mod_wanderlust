use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    pub settings: ControllerSettings,
    pub last_goal_velocity: Vec3,
}

#[derive(Default, Reflect)]
pub struct ControllerSettings {
    pub acceleration: f32,
    pub max_speed: f32,
    pub max_acceleration_force: f32,
    pub jump_force: f32,
    pub force_scale: Vec3,
    pub float_ray_length: f32,
    pub float_distance: f32,
    pub float_ray_dir: Vec3,
    pub float_strength: f32,
    pub float_dampen: f32,
}
