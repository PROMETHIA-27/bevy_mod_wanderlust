use crate::{components::*, systems::*};
use bevy::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin;

impl Plugin for WanderlustPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharacterController>()
            .register_type::<ControllerSettings>()
            .register_type::<ControllerInput>()
            .add_system(movement)
            .add_system_to_stage(CoreStage::PreUpdate, add_settings_and_input);
    }
}
