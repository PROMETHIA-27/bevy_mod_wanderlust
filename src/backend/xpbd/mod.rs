
use bevy::prelude::*;

#[cfg(feature = "xpbd_3d")]
pub use bevy_xpbd_3d as xpbd;
#[cfg(feature = "xpbd_2d")]
pub use bevy_xpbd_2d as xpbd;

use xpbd::prelude::*;

mod mass;
pub use mass::*;
mod velocity;
pub use velocity::*;

pub use xpbd::prelude::Collider;

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct XpbdPhysicsBundle {
    /// See [`RigidBody`].
    pub rigidbody: RigidBody,
    /// See [`Collider`].
    pub collider: Collider,
    /// See [`GravityScale`].
    pub gravity: GravityScale,
    /// See [`Friction`].
    pub friction: Friction,
    /// See [`Restitution`].
    pub restitution: Restitution,
}

impl Default for XpbdPhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: default(),
            collider: Collider::capsule_endpoints(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.5, 0.0),
                0.5,
            ),
            gravity: GravityScale(0.0),
            friction: Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        }
    }
}

pub fn apply_forces() {}
pub fn apply_ground_forces() {}
pub fn setup_physics_context() {}

pub type SpatialQuery<'w, 's> = xpbd::prelude::SpatialQuery<'w, 's>;

use crate::backend::{RayCastResult, Filter};
pub fn cast_ray(
    spatial_query: &SpatialQuery,
    origin: Vec3,
    direction: Vec3,
    max_toi: f32,
    solid: bool,
    filter: Filter,
) -> Option<RayCastResult> {
    spatial_query.cast_ray(
        origin,
        direction,
        max_toi,
        solid,
        SpatialQueryFilter {
            excluded_entities: filter.exclude,
            ..default()
        },
    ).map(|result| {
        let point = origin + direction * result.time_of_impact;
        RayCastResult {
            entity: result.entity,
            normal: result.normal,
            point: point,
            toi: result.time_of_impact,
        }
    })
}
