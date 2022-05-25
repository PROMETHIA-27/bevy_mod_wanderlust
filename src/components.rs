use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CharacterController {
    settings: ControllerSettings,
}

#[derive(Reflect)]
pub struct ControllerSettings {}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {}
    }
}
