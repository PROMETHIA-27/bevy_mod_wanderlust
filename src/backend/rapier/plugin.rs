use crate::*;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*, utils::HashSet};

pub struct WanderlustRapierPlugin {
    pub tweaks: bool,
    pub schedule: Box<dyn ScheduleLabel>,
}

impl Plugin for WanderlustRapierPlugin {
    fn build(&self, app: &mut App) {
        if self.tweaks {
            app.add_systems(Startup, super::setup_physics_context);
        }

        app.configure_sets(
            self.schedule.clone(),
            (WanderlustSet::Apply,).before(crate::rapier::PhysicsSet::SyncBackend),
        );

        app.add_systems(
            self.schedule.clone(),
            (
                super::update_delta_time,
                super::get_mass_from_backend,
                super::get_velocity_from_backend,
            )
                .chain()
                .in_set(WanderlustSet::Sync),
        );

        app.add_systems(
            self.schedule.clone(),
            (super::apply_forces, super::apply_ground_forces)
                .chain()
                .in_set(WanderlustSet::Apply),
        );
    }
}
