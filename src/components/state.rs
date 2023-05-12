use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Vec3};
use bevy::reflect::Reflect;

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
    /// Is the character firmly grounded (and thus able to jump)
    pub is_grounded: bool,
}
