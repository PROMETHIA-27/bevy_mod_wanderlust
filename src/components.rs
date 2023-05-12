use bevy::prelude::{Component, Entity, ReflectComponent, ReflectDefault, Vec3};
use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::{Collider, Toi};

pub mod input;
pub mod settings;
pub mod state;

pub use {input::*, settings::*, state::*};

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Groundcaster {
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
    /// The cached ground cast
    #[reflect(ignore)]
    pub cast: Option<(Entity, Toi)>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration due to gravity
    pub acceleration: Vec3,
    /// Normalized negative acceleration
    pub up_vector: Vec3,
}
