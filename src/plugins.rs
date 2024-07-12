use crate::controller::*;
use bevy::{
    ecs::{intern::Interned, schedule::ScheduleLabel},
    prelude::*,
    utils::HashSet,
};
use bevy_rapier3d::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin {
    tweaks: bool,
    schedule: Interned<dyn ScheduleLabel>,
    default_system_setup: bool,
}

impl WanderlustPlugin {
    /// Apply tweaks to rapier (`true`) to try to avoid some jitters/issues.
    pub fn with_tweaks(mut self, tweaks: bool) -> Self {
        self.tweaks = tweaks;
        self
    }

    /// Specifies whether the plugin should setup each of its [`PhysicsStages`]
    /// (`true`), or if the user will set them up later (`false`).
    ///
    /// The default value is `true`.
    pub fn with_default_system_setup(mut self, default_system_setup: bool) -> Self {
        self.default_system_setup = default_system_setup;
        self
    }

    /// Adds the controller systems to the `FixedUpdate` schedule rather than `Update`.
    pub fn in_fixed_schedule(self) -> Self {
        self.in_schedule(FixedUpdate)
    }

    /// Adds the controller systems to the provided schedule rather than `Update`.
    pub fn in_schedule(mut self, schedule: impl ScheduleLabel) -> Self {
        self.schedule = schedule.intern();
        self
    }
}

impl Default for WanderlustPlugin {
    fn default() -> Self {
        Self {
            tweaks: true,
            schedule: PostUpdate.intern(),
            default_system_setup: true,
        }
    }
}

impl Plugin for WanderlustPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllerInput>()
            .register_type::<Gravity>()
            .register_type::<GravityForce>()
            .register_type::<Movement>()
            .register_type::<MovementForce>()
            .register_type::<Float>()
            .register_type::<FloatForce>()
            .register_type::<Upright>()
            .register_type::<UprightForce>()
            .register_type::<Option<Vec3>>()
            .register_type::<GroundCaster>()
            .register_type::<GroundCast>()
            .register_type::<ViableGroundCast>()
            .register_type::<Grounded>()
            .register_type::<GroundForce>()
            .register_type::<Movement>()
            .register_type::<MovementForce>()
            .register_type::<Jump>()
            .register_type::<JumpForce>()
            .register_type::<Float>()
            .register_type::<FloatForce>()
            .register_type::<Upright>()
            .register_type::<UprightForce>()
            .register_type::<ForceSettings>()
            .register_type::<HashSet<Entity>>();

        if self.tweaks {
            app.add_systems(Startup, setup_physics_context);
        }

        if self.default_system_setup {
            app.add_systems(
                self.schedule.clone(),
                (
                    crate::get_mass_from_rapier,
                    crate::get_velocity_from_rapier,
                    find_ground,
                    determine_groundedness,
                    gravity_force,
                    movement_force,
                    float_force,
                    upright_force,
                    jump_force,
                    accumulate_forces,
                    crate::apply_forces,
                    crate::apply_ground_forces,
                )
                    .chain()
                    .before(PhysicsSet::SyncBackend),
            );
        }

        #[cfg(feature = "debug_lines")]
        app.add_systems(Update, |casts: Query<&GroundCast>, mut gizmos: Gizmos| {
            for cast in &casts {
                if let Some((entity, toi, velocity)) = cast.cast {
                    gizmos.sphere(toi.witness1, Quat::IDENTITY, 0.3, Color::LIME_GREEN);
                }
            }
        });
    }
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// Alternatively, if one only wants to disable the system, use [`WanderlustPhysicsTweaks`](WanderlustPhysicsTweaks).*
///
/// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
pub fn setup_physics_context(/*mut ctx: ResMut<RapierContext>*/) {
    /*
    let params = &mut ctx.integration_parameters;
    // This prevents any noticeable jitter when running facefirst into a wall.
    params.erp = 0.99;
    // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
    params.max_velocity_iterations = 16;
    // TODO: Fix jitter that occurs when running facefirst into a normal corner.
    */
}
