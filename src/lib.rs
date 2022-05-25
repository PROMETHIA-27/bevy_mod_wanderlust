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
            .add_startup_system(setup)
            // .add_system(mouse_look)
            .run()
    }

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
            .insert(Name::from("Player"))
            .with_children(|commands| {
                commands.spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
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
            .insert(Collider::heightfield(
                vec![0.0, 0.0, 0.0, 0.0],
                2,
                2,
                [50.0, 0.0, 50.0].into(),
            ))
            .insert(Name::from("Ground"));
    }

    fn mouse_look(
        mut cam: Query<&mut Transform, With<Camera>>,
        mut input: EventReader<MouseMotion>,
        time: Res<Time>,
    ) {
        let mut tf = cam.single_mut();

        let dt = time.delta_seconds();

        // for delta in input.iter() {
        //     tf.rotate(Quat::from_scaled_axis(Vec3::Y * delta.delta.x * dt));
        //     tf.rotate(Quat::from_scaled_axis(Vec3::X * delta.delta.y * dt));
        // }
    }
}
