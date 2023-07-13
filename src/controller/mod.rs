use bevy::prelude::*;

mod gravity;
mod ground;
mod input;
mod movement;
mod orientation;

use crate::physics::*;
use crate::Spring;

pub use {gravity::*, ground::*, input::*, movement::*, orientation::*};

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ForceSettings {
    /// Scaling factor for the force applied to the ground to keep the character moving/off the ground.
    pub opposing_force_scale: f32,
    /// Scaling factor for the movement impulse applied to the ground.
    /// Setting this to 0.0 would make it so things don't "slip" out from the characters feet.
    pub opposing_movement_force_scale: f32,
}

pub fn accumulate_forces(
    globals: Query<&GlobalTransform>,
    mut forces: Query<(
        &ForceSettings,
        &mut ControllerForce,
        &mut GroundForce,
        &FloatForce,
        &UprightForce,
        &MovementForce,
        &JumpForce,
        &GravityForce,
        &GroundCast,
        &ControllerMass,
    )>,
) {
    for (
        settings,
        mut force,
        mut ground_force,
        float,
        upright,
        movement,
        jump,
        gravity,
        ground_cast,
        mass,
    ) in &mut forces
    {
        force.linear = movement.linear + jump.linear + float.linear + gravity.linear;
        force.angular = upright.angular;

        let opposing_force = -(movement.linear * settings.opposing_movement_force_scale
            + (jump.linear + float.linear) * settings.opposing_force_scale);

        if let Some((ground_entity, toi, velocity)) = ground_cast {
            let ground_transform = match globals.get(ground_entity) {
                Ok(global) => global.compute_transform(),
                _ => Transform::default(),
            };

            let point = ground_transform.inverse() * toi.witness1;
            ground_force.linear = opposing_force;
            ground_force.angular = (point - mass.com).cross(opposing_force),

            #[cfg(feature = "debug_lines")]
            {
                let color = if opposing_impulse.dot(settings.up_vector) < 0.0 {
                    Color::RED
                } else {
                    Color::BLUE
                };
                gizmos.line(toi.witness1, toi.witness1 + opposing_impulse, color);
            }
        } else {
            ground_force.linear = opposing_force;
            ground_force.angular = Vec3::ZERO;
        }
    }
}
