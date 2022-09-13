use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_mod_wanderlust::backends::{Rapier2dBackend, Rapier2dControllerPhysicsBundle};
use bevy_mod_wanderlust::{ControllerInput, StarshipControllerBundle, WanderlustPlugin};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::Damping;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WanderlustPlugin(Rapier2dBackend))
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, input)
        .register_type::<Player>()
        .run();
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // Origin cube to be able to tell how you're moving
    let mesh = meshes.add(shape::Cube { size: 10.0 }.into());
    let mat = mats.add(Color::WHITE.into());

    c.spawn_bundle(PbrBundle {
        mesh,
        material: mat.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..default()
    });

    // Light so you can see the cube
    c.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(15.0, 16.0, 17.0),
        point_light: PointLight {
            color: Color::default(),
            intensity: 8000.0,
            range: 50.0,
            ..default()
        },
        ..default()
    });

    // The ship itself
    c.spawn_bundle(StarshipControllerBundle::<Rapier2dBackend> {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        physics: Rapier2dControllerPhysicsBundle {
            damping: Damping {
                angular_damping: 0.5,
                linear_damping: 0.5,
            },
            ..default()
        },
        ..default()
    })
    .insert_bundle((Player,))
    .with_children(|c| {
        c.spawn_bundle(SceneBundle {
            transform: Transform::from_translation(Vec3::ZERO).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                FRAC_PI_2,
                -FRAC_PI_2,
                0.0,
            )),
            scene: ass.load("gltf/starship.glb#Scene0"),
            ..default()
        });
    });
    c.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });
}

fn input(
    mut body: Query<(&mut ControllerInput, &GlobalTransform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const ROLL_MULT: f32 = 1.0;

    let (mut body, tf) = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::Q) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::E) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.up();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.up();
    }

    body.movement = dir;

    let dt = time.delta_seconds();
    if input.pressed(KeyCode::A) {
        body.custom_torque += -tf.forward() * dt * ROLL_MULT;
    }
    if input.pressed(KeyCode::D) {
        body.custom_torque += tf.forward() * dt * ROLL_MULT;
    }
}
