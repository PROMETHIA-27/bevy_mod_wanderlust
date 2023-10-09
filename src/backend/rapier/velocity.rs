
use bevy::{
    prelude::*,
    ecs::query::WorldQuery,
};
use super::rapier::prelude::*;
use crate::*;

pub fn get_velocity_from_backend(
    mut query: Query<(&mut ControllerVelocity, &Velocity)>,
) {
    for (mut velocity, rapier) in &mut query {
        velocity.linear = rapier.linvel;
        velocity.angular = rapier.angvel;
    }
}
