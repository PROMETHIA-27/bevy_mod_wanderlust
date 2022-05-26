mod bundles;
mod components;
mod plugins;
mod systems;

pub use self::{
    bundles::CharacterControllerBundle,
    components::{CharacterController, ControllerSettings},
    plugins::WanderlustPlugin,
};

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{input::mouse::MouseMotion, prelude::*};
    use bevy_editor_pls::prelude::*;
    use bevy_rapier3d::prelude::*;

    #[test]
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(EditorPlugin)
            .insert_resource(Sensitivity(0.3))
            .add_startup_system(setup)
            .add_system(mouse_look)
            .run()
    }

    #[derive(Component, Default, Reflect)]
    #[reflect(Component)]
    struct PlayerCam;

    #[derive(Component, Default, Reflect)]
    #[reflect(Component)]
    struct PlayerBody;

    #[derive(Reflect)]
    struct Sensitivity(f32);

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut mats: ResMut<Assets<StandardMaterial>>,
    ) {
        let mesh = meshes.add(
            shape::Capsule {
                radius: 0.5,
                depth: 1.0,
                ..default()
            }
            .into(),
        );

        let material = mats.add(Color::WHITE.into());

        commands
            .spawn_bundle(CharacterControllerBundle::default())
            .insert_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                ..default()
            })
            .insert_bundle((Name::from("Player"), PlayerBody))
            .with_children(|commands| {
                commands
                    .spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        ..default()
                    })
                    .insert(PlayerCam)
                    .with_children(|commands| {
                        let mesh = meshes.add(shape::Cube { size: 0.5 }.into());

                        commands.spawn_bundle(PbrBundle {
                            mesh,
                            material: material.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.5),
                            ..default()
                        });
                    });
            });

        let mesh = meshes.add(shape::Plane { size: 10.0 }.into());

        commands
            .spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(0.0, -10.0, 0.0),
                ..default()
            })
            .insert_bundle((
                Collider::heightfield(vec![0.0, 0.0, 0.0, 0.0], 2, 2, [50.0, 0.0, 50.0].into()),
                Name::from("Ground"),
            ));

        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        });
    }

    fn mouse_look(
        mut cam: Query<&mut Transform, With<PlayerCam>>,
        mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCam>)>,
        sensitivity: Res<Sensitivity>,
        mut input: EventReader<MouseMotion>,
        time: Res<Time>,
    ) {
        let mut cam_tf = cam.single_mut();
        let mut body_tf = body.single_mut();

        let dt = time.delta_seconds();
        let sens = sensitivity.0;

        for motion in input.iter() {
            let rot = cam_tf.rotation;
            cam_tf.rotate(Quat::from_scaled_axis(
                rot * Vec3::X * -motion.delta.y * dt * sens,
            ));

            let rot = body_tf.rotation;
            body_tf.rotate(Quat::from_scaled_axis(
                rot * Vec3::Y * -motion.delta.x * dt * sens,
            ));
        }
    }
}
