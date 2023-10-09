
use bevy::{
    ecs::query::WorldQuery,
    prelude::*,
};
use super::rapier;

#[derive(WorldQuery)]
pub struct Mass {
    mass_properties: &'static rapier::prelude::ReadMassProperties,
}

impl<'a> MassItem<'a> {
    pub fn mass(&self) -> f32 {
        self.mass_properties.0.mass
    }

    pub fn inertia(&self) -> Vec3 {
        self.mass_properties.0.principal_inertia
    }

    pub fn inertia_matrix(&self) -> Mat3 {
        self.mass_properties.0.into_rapier(1.0).reconstruct_inertia_matrix().into()
    }

    pub fn local_center_of_mass(&self) -> Vec3 {
        self.mass_properties.0.local_center_of_mass
    }
}