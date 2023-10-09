use super::xpbd;
use bevy::{ecs::query::WorldQuery, prelude::*};

#[derive(WorldQuery)]
pub struct Velocity {
    linear: &'static xpbd::prelude::LinearVelocity,
    angular: &'static xpbd::prelude::AngularVelocity,
}

impl<'a> VelocityItem<'a> {
    pub fn linear(&self) -> Vec3 {
        **self.linear
    }

    pub fn angular(&self) -> Vec3 {
        **self.angular
    }
}
