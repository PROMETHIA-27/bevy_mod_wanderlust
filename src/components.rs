use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    pub settings: ControllerSettings,
}

#[derive(Default, Reflect)]
pub struct ControllerSettings {
    pub acceleration: f32,
    pub jump_force: f32,
    pub float_ray_length: f32,
    pub float_distance: f32,
    pub float_ray_dir: Vec3,
    pub float_strength: f32,
    pub float_dampen: f32,
}
