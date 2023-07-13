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
}
