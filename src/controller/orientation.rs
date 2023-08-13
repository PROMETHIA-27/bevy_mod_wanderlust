use crate::controller::*;
/// Keeps the controller properly oriented in a floating state.

#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Float {
    /// How far to attempt to float away from the ground.
    pub distance: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much lower than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub min_offset: f32,
    /// While floating, the character can be floating at a different exact distance than [`float_distance`] depending on other forces acting on them.
    /// This field controls how much higher than [`float_distance`] they can be and still be considered grounded.
    ///
    /// This helps keep jumps more consistent when the ground cast length is longer than the float distance.
    pub max_offset: f32,
    /// How strongly to float away from the ground.
    pub spring: Spring,
}

impl Default for Float {
    fn default() -> Self {
        Self {
            distance: 0.55,
            min_offset: -0.3,
            max_offset: 0.05,
            spring: Spring {
                strength: 100.0,
                damping: 0.8,
            },
        }
    }
}

/// Force applied to push the controller off the ground.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct FloatForce {
    /// Linear force.
    pub linear: Vec3,
}

/// Calculate "floating" force, as seen [here](https://www.youtube.com/watch?v=qdskE8PJy6Q)
pub fn float_force(
    mut query: Query<(
        &GlobalTransform,
        &mut FloatForce,
        &Float,
        &GroundCast,
        &ControllerVelocity,
        &ControllerMass,
        &Gravity,
    )>,
) {
    for (global, mut force, float, cast, velocity, mass, gravity) in &mut query {
        force.linear = Vec3::ZERO;

        //let ViableGround::Ground(ground) = cast.viable else { continue };
        let Some(ground) = cast.current else { continue };
        if !ground.viable { continue }

        let up_vector = gravity.up_vector;

        let controller_point_velocity =
            velocity.linear + velocity.angular.cross(Vec3::ZERO - mass.com);
        let vel_align = up_vector.dot(controller_point_velocity);
        let ground_vel_align = up_vector.dot(ground.point_velocity.linvel);

        let relative_velocity = vel_align - ground_vel_align;

        let worldspace_diff = global.translation().dot(gravity.up_vector) - ground.cast.point.dot(gravity.up_vector);
        let displacement = float.distance - worldspace_diff;

        if displacement > 0.0 {
            let strength = displacement * float.spring.strength;
            let damping = relative_velocity * float.spring.damp_coefficient(mass.mass);
            force.linear += up_vector * (strength - damping);
        }
    }
}

/// How to keep the controller upright, as well as
/// facing a specific direction.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Upright {
    /// How strongly to force the character upright/avoid overshooting. Alternatively, see [`LockedAxes`] to lock rotation entirely.
    pub spring: Spring,
    /// The direction to face towards, or `None` to not rotate to face any direction. Must be perpendicular to the up vector and normalized.
    pub forward_vector: Option<Vec3>,
}

impl Default for Upright {
    fn default() -> Self {
        Self {
            spring: Spring {
                strength: 10.0,
                damping: 0.5,
            },
            forward_vector: None,
        }
    }
}

/// Forces applied to keep the controller upright and optionally facing a direction.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UprightForce {
    /// Angular force.
    pub angular: Vec3,
}

/// Make sure the controller stays upright/does not tilt or fall over on its side.
pub fn upright_force(
    mut query: Query<(
        &mut UprightForce,
        &Upright,
        &GlobalTransform,
        &Gravity,
        &ControllerMass,
        &ControllerVelocity,
        &GroundCast,
    )>,
) {
    for (mut impulse, upright, tf, gravity, mass, velocity, ground_cast) in &mut query {
        impulse.angular = {
            let desired_axis = if let Some(forward) = upright.forward_vector {
                let right = gravity.up_vector.cross(forward).normalize();
                let up = forward.cross(right);
                let target_rot = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
                let current = tf.to_scale_rotation_translation().1;
                let rot = target_rot * current.inverse();
                let (axis, mut angle) = rot.to_axis_angle();
                if angle > std::f32::consts::PI {
                    angle -= 2.0 * std::f32::consts::PI;
                }
                axis * angle
            } else {
                let current = tf.up();
                current.cross(gravity.up_vector)
            };

            //upright.spring.damping = 0.1;

            let damping = Vec3::new(
                upright.spring.damp_coefficient(mass.inertia.x),
                upright.spring.damp_coefficient(mass.inertia.y),
                upright.spring.damp_coefficient(mass.inertia.z),
            );

            let ground_rot = if let Some(ground) = ground_cast.viable.last() {
                ground.point_velocity.angvel
                //Vec3::ZERO
            } else {
                Vec3::ZERO
            };

            let local_velocity = velocity.angular - ground_rot;
            let projected_vel = if local_velocity.length() > 0.0 && desired_axis.length() > 0.0 {
                local_velocity.project_onto(desired_axis)
            } else {
                Vec3::ZERO
            };

            let spring = (desired_axis * upright.spring.strength) - (velocity.angular * damping);
            spring.clamp_length_max(upright.spring.strength)
        };
    }
}
