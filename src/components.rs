use bevy::prelude::{Component, Entity, ReflectComponent, ReflectDefault, Vec3};
use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::{Collider, Toi};

use crate::Spring;

pub mod input;
pub mod settings;
pub mod state;

pub use {input::*, settings::*, state::*};

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GroundCaster {
    /// A timer to track how long to skip the ground check for (see [`jump_skip_ground_check_duration`](ControllerSettings::jump_skip_ground_check_duration)).
    pub skip_ground_check_timer: f32,
    /// Override skip ground check. If true, never checks for the ground.
    pub skip_ground_check_override: bool,
    /// An offset to start the ground check from, relative to the character's origin.
    pub cast_origin: Vec3,
    /// How long of a ray to cast to detect the ground. Setting this unnecessarily high will permanently count the player as grounded,
    /// and too low will allow the player to slip and become disconnected from the ground easily.
    pub cast_length: f32,
    /// What shape of ray to cast. See [`Collider`] and [`RapierContext::cast_shape`](bevy_rapier::prelude::RapierContext).
    #[reflect(ignore)]
    pub cast_collider: Collider,
    /// Set of entities that should be ignored when ground casting.
    pub exclude_from_ground: HashSet<Entity>,
    /// The maximum angle that the ground can be, in radians, before it is no longer considered suitable for being "grounded" on.
    ///
    /// For example, if this is set to `π/4` (45 degrees), then a player standing on a slope steeper than 45 degrees will slip and fall, and will not have
    /// their jump refreshed by landing on that surface.
    ///
    /// This is done by ignoring the ground during ground casting.
    pub max_ground_angle: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GroundCast {
    /// The cached ground cast. Contains the entity hit, the hit info, and velocity of the entity
    /// hit.
    #[reflect(ignore)]
    pub cast: Option<(Entity, Toi, Velocity)>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Grounded {
    /// Is the character grounded?
    pub grounded: bool,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration due to gravity
    pub acceleration: Vec3,
    /// Normalized negative acceleration
    pub up_vector: Vec3,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Float {
    /// How far to attempt to float away from the ground.
    pub distance: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much lower than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub min_offset: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much higher than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub max_offset: f32,
    /// How strongly to float away from the ground.
    pub spring: Spring,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct FloatForce {
    /// The contribution of float force to the final motion
    pub force: Vec3,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ExtraJumps {
    /// How many extra times the character can jump after leaving the ground. 0 is normal, 1 corresponds to double jump, etc.
    pub extra: u32,
    /// How many extra jumps are remaining
    pub remaining: u32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct CoyoteTime {
    /// How long should the character still be able to jump after leaving the ground, in seconds.
    /// For example, if this is set to 0.5, the player can fall off a ledge and then jump if they do so within 0.5 seconds of leaving the ledge.
    pub duration: f32,
    /// A timer to track coyote time. See [`coyote_time_duration`](Self::coyote_time_duration)
    pub timer: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct FinalMotion {
    /// The total impulse from all sources, which will make the character move
    pub total: Vec3,
    /// The impulse from the character itself, which will affect things around it
    /// (newton's second law)
    pub internal: Vec3,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Mass {
    /// The mass of a character
    pub mass: f32,
    /// The rotational inertia of a character
    pub inertia: Vec3,
    /// The center of mass of a character
    pub com: Vec3,
}

#[derive(Copy, Clone, Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Velocity {
    /// How fast this character is currently moving linearly in 3D space
    pub lin: Vec3,
    /// How fast this character is currently moving rotationally in 3D space
    pub ang: Vec3,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ContinuousMovement {
    pub acceleration: f32,
    pub max_acceleration_force: f32,
    pub max_speed: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Jumping {
    /// Was [`ControllerInput::jumping`] true last frame.
    pub jump_pressed_last_frame: bool,
    /// A timer to track how long to jump for.
    pub jump_timer: f32,
    /// A timer to track jump buffering. See [`jump_buffer_duration`](ControllerSettings::jump_buffer_duration)
    pub jump_buffer_timer: f32,
    /// The amount of force to apply on the first frame when a jump begins.
    pub jump_initial_force: f32,
    /// The amount of force to continuously apply every second during a jump.
    pub jump_force: f32,
    /// The amount of force to apply downwards when the jump control is released prior to a jump expiring.
    /// This allows analog jumping by cutting the jump short when the control is released.
    pub jump_stop_force: f32,
    /// How long a jump can last.
    pub jump_time: f32,
    /// If the jump input is pressed before landing, how long will the jump be buffered for?
    /// In other words, if this is 0.5, the character can input jump up to 0.5 seconds before landing and the jump will occur when they land.
    pub jump_buffer_duration: f32,
    /// A function taking the current progress of a jump, from 0.0 to 1.0, with 0.0 indicating a jump has just begun and 1.0 indicating the jump has ended,
    /// which returns a modifier (usually from 0.0 to 1.0, but not necessarily) to multiply [`jump_force`](ControllerSettings::jump_force) by.
    #[reflect(ignore)]
    pub jump_decay_function: Option<fn(f32) -> f32>,
    /// How long to skip ground checks after jumping. Usually this should be set just high enough that the character is out of range of the ground
    /// just before the timer elapses.
    pub jump_skip_ground_check_duration: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UprightSpring {
    /// How strongly to force the character upright/avoid overshooting. Alternatively, see [`LockedAxes`] to lock rotation entirely.
    pub upright_spring: Spring,
    /// The direction to face towards, or `None` to not rotate to face any direction. Must be perpendicular to the up vector and normalized.
    pub forward_vector: Option<Vec3>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UprightForce {
    /// The contribution of upright force to the final motion
    pub force: Vec3,
}
