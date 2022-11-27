use bevy::prelude::Resource;

/// Should [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) tweak physics rules to make the character controller work better?
/// If not present, defaults to true. Must be added before [`WanderlustPlugin`](crate::plugins::WanderlustPlugin).
#[derive(Resource)]
pub struct WanderlustPhysicsTweaks(#[deprecated] pub bool);

impl WanderlustPhysicsTweaks {
    /// Construct a new [`WanderlustPhysicsTweaks`]. `do_tweaks` controls whether or not physics tweaks will be applied
    /// by [`WanderlustPlugin`](crate::plugins::WanderlustPlugin).
    pub fn new(do_tweaks: bool) -> Self {
        Self(do_tweaks)
    }

    /// Will the [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) tweak rapier physics settings?
    #[allow(deprecated)]
    pub fn should_do_tweaks(&self) -> bool {
        self.0
    }
}
