use crate::{controller::GroundCast, physics::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Contains common physics settings for character controllers.
#[derive(Bundle)]
pub struct RapierPhysicsBundle {
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

impl Default for RapierPhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: default(),
            collider: Collider::capsule(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.5, 0.0), 0.5),
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
            read_mass_properties: default(),
        }
    }
}

pub fn apply_forces(
    mut forces: Query<(&mut ExternalImpulse, &ControllerForce)>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (mut impulse, force) in &mut forces {
        impulse.impulse += force.linear * dt;
        impulse.torque_impulse += force.angular * dt;
    }
}

pub fn apply_ground_forces(
    mut impulses: Query<&mut ExternalImpulse>,
    ground_forces: Query<(&GroundForce, &GroundCast)>,
    ctx: Res<RapierContext>,
) {
    let dt = ctx.integration_parameters.dt;
    for (force, cast) in &ground_forces {
        if let Some((ground, toi, velocity)) = cast.cast {
            if let Ok(mut impulse) = impulses.get_mut(ground) {
                impulse.impulse += force.linear * dt;
                impulse.torque_impulse += force.angular * dt;
            }
        }
    }
}

pub fn get_mass_from_rapier(
    mut query: Query<(&mut ControllerMass, &ReadMassProperties), Changed<ReadMassProperties>>,
) {
    for (mut mass, rapier_mass) in &mut query {
        mass.mass = rapier_mass.0.mass;
        mass.inertia = rapier_mass.0.principal_inertia;
        mass.com = rapier_mass.0.local_center_of_mass;
    }
}

pub fn get_velocity_from_rapier(
    mut query: Query<(&mut ControllerVelocity, &Velocity), Changed<Velocity>>,
) {
    for (mut vel, rapier_vel) in &mut query {
        vel.linear = rapier_vel.linvel;
        vel.angular = rapier_vel.angvel;
    }
}
