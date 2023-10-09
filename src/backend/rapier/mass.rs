use super::rapier::prelude::*;
use crate::*;
use bevy::{ecs::query::WorldQuery, prelude::*};

pub fn get_mass_from_backend(mut query: Query<(&mut ControllerMass, &ReadMassProperties)>) {
    for (mut mass, rapier) in &mut query {
        *mass = ControllerMass::from_rapier(&*rapier);
    }
}

impl ControllerMass {
    pub fn from_rapier(rapier: &MassProperties) -> Self {
        Self {
            mass: rapier.mass,
            inertia: rapier.principal_inertia,
            local_center_of_mass: rapier.local_center_of_mass,
        }
    }
}
