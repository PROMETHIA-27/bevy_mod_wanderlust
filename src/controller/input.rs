use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Vec3};
use bevy::reflect::Reflect;

/// This is the interface for applying input to the character controller.
/// See each field for more information.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ControllerInput {
    /// This field represents movement in 3D space.
    /// The majority of games will map this to WASD/Analog joystick in 2D space along the ground.
    /// To ensure movement does not affect the Y axis, set [`ControllerSettings::force_scale`] to `Vec3(1.0, 0.0, 1.0)`.
    pub movement: Vec3,
    /// This field represents if the jump control is currently pressed.
    pub jumping: bool,
}
