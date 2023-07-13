use crate::controller::ControllerInput;
use bevy::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin;

impl Plugin for WanderlustPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllerInput>()
            .register_type::<Option<Vec3>>();
        //.add_systems(Startup, setup_physics_context)
        //.add_systems(Update, movement);
    }
}
