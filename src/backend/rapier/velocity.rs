
use bevy::{
    prelude::*,
    ecs::query::WorldQuery,
};
use super::rapier;

#[derive(WorldQuery)]
pub struct Velocity {
    velocity: &'static rapier::prelude::Velocity,
}

impl<'a> VelocityItem<'a> {
    pub fn linear(&self) -> Vec3 {
        self.velocity.linvel
    }

    pub fn angular(&self) -> Vec3 {
        self.velocity.angvel
    }
}