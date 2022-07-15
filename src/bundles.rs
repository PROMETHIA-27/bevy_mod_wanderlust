use super::components::*;
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

/// The recommended bundle for creating a character controller. Includes the necessary components for a character controller
/// as well as many physics-related components that can be used to tweak the behavior of the controller, with reasonable default
/// values.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    /// See [`CharacterController`].
    pub controller: CharacterController,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`RigidBody`].
    pub rigidbody: RigidBody,
    /// See [`Collider`].
    pub collider: Collider,
    /// See [`Transform`].
    pub transform: Transform,
    /// See [`Velocity`].
    pub velocity: Velocity,
    /// See [`GravityScale`].
    pub gravity: GravityScale,
    /// See [`Sleeping`].
    pub sleeping: Sleeping,
    /// See [`Ccd`].
    pub ccd: Ccd,
    /// See [`ExternalImpulse`].
    pub force: ExternalImpulse,
    /// See [`LockedAxes`].
    pub locked_axes: LockedAxes,
    /// See [`Friction`].
    pub friction: Friction,
    /// See [`Damping`].
    pub damping: Damping,
    /// See [`Restitution`].
    pub restitution: Restitution,
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            settings: ControllerSettings {
                acceleration: 50.0,
                max_speed: 10.0,
                max_acceleration_force: 10.0,
                up_vector: Vec3::Y,
                gravity: 25.0,
                max_ground_angle: 45.0 * (std::f32::consts::PI / 180.0),
                jump_time: 0.5,
                jump_initial_force: 15.0,
                jump_force: 0.0,
                jump_stop_force: 0.3,
                jump_decay_function: |x| (1.0 - x).sqrt(),
                jump_skip_ground_check_duration: 0.5,
                coyote_time_duration: 0.16,
                jump_buffer_duration: 0.16,
                force_scale: vec3(1.0, 0.0, 1.0),
                float_cast_length: 1.0,
                float_cast_origin: vec3(0.0, 0.0, 0.0),
                float_cast_collider: Collider::ball(0.45),
                float_distance: 0.55,
                float_strength: 10.0,
                float_dampen: 0.5,
                upright_spring_strength: 10.0,
                upright_spring_damping: 2.0,
            },
            input: default(),
            rigidbody: default(),
            collider: Collider::capsule(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.5, 0.0), 0.5),
            transform: default(),
            velocity: default(),
            gravity: GravityScale(0.0),
            sleeping: default(),
            ccd: default(),
            force: default(),
            locked_axes: default(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}
