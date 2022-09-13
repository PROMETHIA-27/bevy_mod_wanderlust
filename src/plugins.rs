use crate::{components::*, systems::*};
use bevy::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin<B: crate::PhysicsBackend>(pub B);

impl<B: crate::PhysicsBackend> Plugin for WanderlustPlugin<B> {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllerState>()
            .register_type::<ControllerSettings<B::CastableShape>>()
            .register_type::<ControllerInput>()
            .add_startup_system_set(B::generate_setup_system_set())
            .add_system(movement::<B>);
    }
}
