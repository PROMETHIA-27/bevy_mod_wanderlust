//! Implementations specific to the 3D version of Rapier.

use super::{BackendCastableShape, PhysicsBackend, TOIStatusProxy, ToiProxy};
use bevy::{math::*, prelude::*};
use bevy_rapier3d::prelude::*;

/// A [PhysicsBackend] for using the 3D version of Rapier.
pub struct Rapier3dBackend;

impl PhysicsBackend for Rapier3dBackend {
    type ControllerPhysicsBundle = Rapier3dControllerPhysicsBundle;
    type ExternalImpulse = ExternalImpulse;
    type Velocity = Velocity;
    type PhysicsContext = RapierContext;
    type CastableShape = Collider;

    /// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
    fn generate_setup_system_set() -> SystemSet {
        fn setup_physics_context(
            mut ctx: ResMut<RapierContext>,
            should_change: Option<Res<crate::WanderlustPhysicsTweaks>>,
        ) {
            if should_change.map(|s| s.should_do_tweaks()).unwrap_or(true) {
                let params = &mut ctx.integration_parameters;
                // This prevents any noticeable jitter when running facefirst into a wall.
                params.erp = 0.99;
                // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
                params.max_velocity_iterations = 16;
                // TODO: Fix jitter that occurs when running facefirst into a normal corner.
            }
        }
        SystemSet::new().with_system(setup_physics_context)
    }

    fn apply_impulses(body: &mut Self::ExternalImpulse, impulse: Vec3, torque_impulse: Vec3) {
        body.impulse = impulse;
        body.torque_impulse = torque_impulse;
    }

    fn entity_has_contacts(ctx: &Self::PhysicsContext, entity: Entity) -> bool {
        ctx.contacts_with(entity).next().is_some()
    }

    fn cast_shape(
        ctx: &Self::PhysicsContext,
        transofrm: &GlobalTransform,
        settings: &crate::ControllerSettings<Self::CastableShape>,
        entity: Entity,
    ) -> Option<(Entity, ToiProxy)> {
        ctx.cast_shape(
            transofrm.mul_vec3(settings.float_cast_origin),
            transofrm.to_scale_rotation_translation().1,
            -settings.up_vector,
            &settings.float_cast_collider,
            settings.float_cast_length,
            QueryFilter::new().predicate(&|collider| collider != entity),
        )
        .map(|(entity, toi)| {
            (
                entity,
                ToiProxy {
                    toi: toi.toi,
                    normal1: toi.normal1,
                    status: match toi.status {
                        TOIStatus::OutOfIterations => TOIStatusProxy::OutOfIterations,
                        TOIStatus::Converged => TOIStatusProxy::Converged,
                        TOIStatus::Failed => TOIStatusProxy::Failed,
                        TOIStatus::Penetrating => TOIStatusProxy::Penetrating,
                    },
                },
            )
        })
    }

    fn extract_linvel(velocity: &Self::Velocity) -> Vec3 {
        velocity.linvel
    }

    fn extract_angvel(velocity: &Self::Velocity) -> Vec3 {
        velocity.angvel
    }
}

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct Rapier3dControllerPhysicsBundle {
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
}

impl Default for Rapier3dControllerPhysicsBundle {
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
        }
    }
}

impl BackendCastableShape for Collider {
    fn ball(radius: f32) -> Self {
        Collider::ball(radius)
    }
}
