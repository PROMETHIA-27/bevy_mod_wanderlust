use crate::{ControllerInput, ControllerSettings, ControllerState};

use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct ControllerPhysicsBundle {
    /// See [`RigidBody`].
    pub rigidbody: RigidBody,
    /// See [`Collider`].
    pub collider: Collider,
    /// See [`Velocity`].
    pub velocity: Velocity,
    /// See [`GravityScale`].
    pub gravity: GravityScale,
    /// See [`Sleeping`].
    pub sleeping: Sleeping,
    /// See [`Ccd`].
    pub ccd: Ccd,
    /// See [`ExternalImpulse`].
    pub force: ExternalImpulse,
    /// See [`LockedAxes`].
    pub locked_axes: LockedAxes,
    /// See [`Friction`].
    pub friction: Friction,
    /// See [`Damping`].
    pub damping: Damping,
    /// See [`Restitution`].
    pub restitution: Restitution,
    /// See [`ReadMassProperties`].
    pub read_mass_properties: ReadMassProperties,
}

impl Default for ControllerPhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: default(),
            collider: Collider::capsule(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.5, 0.0), 0.5),
            velocity: default(),
            gravity: GravityScale(0.0),
            sleeping: default(),
            ccd: default(),
            force: default(),
            locked_axes: default(),
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            read_mass_properties: ReadMassProperties::default(),
        }
    }
}

/// The recommended bundle for creating a basic, walking character controller. Includes the necessary components for a character controller
/// as well as many physics-related components that can be used to tweak the behavior of the controller, with reasonable default
/// values.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    /// See [`ControllerState`].
    pub controller: ControllerState,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`ControllerPhysicsBundle`]
    #[bundle]
    pub physics: ControllerPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`ComputedVisibility`]
    pub computed_visibility: ComputedVisibility,
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            settings: ControllerSettings::character(),
            input: default(),
            physics: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

/// A flying character controller with spaceship-like controls.
#[derive(Bundle)]
pub struct StarshipControllerBundle {
    /// See [`ControllerState`].
    pub controller: ControllerState,
    /// See [`ControllerSettings`].
    pub settings: ControllerSettings,
    /// See [`ControllerInput`].
    pub input: ControllerInput,
    /// See [`ControllerPhysicsBundle`].
    #[bundle]
    pub physics: ControllerPhysicsBundle,
    /// See [`Transform`]
    pub transform: Transform,
    /// See [`GlobalTransform`]
    pub global_transform: GlobalTransform,
    /// See [`Visibility`]
    pub visibility: Visibility,
    /// See [`ComputedVisibility`]
    pub computed_visibility: ComputedVisibility,
}

impl Default for StarshipControllerBundle {
    fn default() -> Self {
        Self {
            controller: default(),
            settings: ControllerSettings::starship(),
            input: default(),
            physics: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

// pub struct ControllerBuilderBundle {
//     /// See [`RigidBody`].
//     pub rigidbody: Option<RigidBody>,
//     /// See [`Collider`].
//     pub collider: Option<Collider>,
//     /// See [`Velocity`].
//     pub velocity: Option<Velocity>,
//     /// See [`GravityScale`].
//     pub gravity: Option<GravityScale>,
//     /// See [`Sleeping`].
//     pub sleeping: Option<Sleeping>,
//     /// See [`Ccd`].
//     pub ccd: Option<Ccd>,
//     /// See [`ExternalImpulse`].
//     pub force: Option<ExternalImpulse>,
//     /// See [`LockedAxes`].
//     pub locked_axes: Option<LockedAxes>,
//     /// See [`Friction`].
//     pub friction: Option<Friction>,
//     /// See [`Damping`].
//     pub damping: Option<Damping>,
//     /// See [`Restitution`].
//     pub restitution: Option<Restitution>,
//     /// See [`ControllerState`].
//     pub controller: Option<ControllerState>,
//     /// See [`ControllerSettings`].
//     pub settings: Option<ControllerSettings>,
//     /// See [`ControllerInput`].
//     pub input: Option<ControllerInput>,
//     /// See [`Transform`]
//     pub transform: Option<Transform>,
//     /// See [`GlobalTransform`]
//     pub global_transform: Option<GlobalTransform>,
//     /// See [`Visibility`]
//     pub visibility: Option<Visibility>,
//     /// See [`ComputedVisibility`]
//     pub computed_visibility: Option<ComputedVisibility>,
// }

// impl ControllerBuilderBundle {
//     fn insert<'w, 's, 'a>(
//         self,
//         mut entity: EntityCommands<'w, 's, 'a>,
//     ) -> EntityCommands<'w, 's, 'a> {
//         if let Some(value) = self.rigidbody {
//             entity.insert(value);
//         }
//         if let Some(value) = self.collider {
//             entity.insert(value);
//         }
//         if let Some(value) = self.velocity {
//             entity.insert(value);
//         }
//         if let Some(value) = self.gravity {
//             entity.insert(value);
//         }
//         if let Some(value) = self.sleeping {
//             entity.insert(value);
//         }
//         if let Some(value) = self.ccd {
//             entity.insert(value);
//         }
//         if let Some(value) = self.force {
//             entity.insert(value);
//         }
//         if let Some(value) = self.locked_axes {
//             entity.insert(value);
//         }
//         if let Some(value) = self.friction {
//             entity.insert(value);
//         }
//         if let Some(value) = self.damping {
//             entity.insert(value);
//         }
//         if let Some(value) = self.restitution {
//             entity.insert(value);
//         }
//         if let Some(value) = self.controller {
//             entity.insert(value);
//         }
//         if let Some(value) = self.settings {
//             entity.insert(value);
//         }
//         if let Some(value) = self.input {
//             entity.insert(value);
//         }
//         if let Some(value) = self.transform {
//             entity.insert(value);
//         }
//         if let Some(value) = self.global_transform {
//             entity.insert(value);
//         }
//         if let Some(value) = self.visibility {
//             entity.insert(value);
//         }
//         if let Some(value) = self.computed_visibility {
//             entity.insert(value);
//         }
//         entity
//     }

//     fn spawn<'w, 's, 'a>(self, c: &'a mut Commands<'w, 's>) -> EntityCommands<'w, 's, 'a> {
//         self.insert(c.spawn())
//     }
// }
