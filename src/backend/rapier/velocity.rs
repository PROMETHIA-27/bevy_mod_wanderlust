use super::rapier::prelude::*;
use crate::*;
use bevy::{ecs::query::WorldQuery, prelude::*};

pub fn get_velocity_from_backend(mut query: Query<(&mut ControllerVelocity, &Velocity)>) {
    for (mut velocity, rapier) in &mut query {
        velocity.linear = rapier.linvel;
        velocity.angular = rapier.angvel;
    }
}
