use crate::{components::*, systems::*};
use bevy::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin;

impl Plugin for WanderlustPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllerState>()
            .register_type::<ControllerSettings>()
            .register_type::<ControllerInput>()
            .register_type::<Option<Vec3>>()
            .add_startup_system(setup_physics_context)
            .add_system(movement);
    }
}
