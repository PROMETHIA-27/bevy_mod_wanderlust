use std::f32::consts::FRAC_PI_2;

use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_mod_wanderlust::{ControllerInput, SpaceshipControllerBundle, WanderlustPlugin};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WanderlustPlugin)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, input)
        .add_system(rotate_to_goal)
        .register_type::<RotateToGoal>()
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct RotateToGoal {
    destination: Quat,
    sensitivity: f32,
    acceleration: f32,
    damping: f32,
}

impl Default for RotateToGoal {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            destination: Quat::default(),
            acceleration: 1.0,
            damping: 1.0,
        }
    }
}

fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // Origin cube to be able to tell how you're moving
    let mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    let mat = mats.add(Color::WHITE.into());

    c.spawn_bundle(PbrBundle {
        mesh,
        material: mat.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Light so you can see the cube
    c.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
        ..default()
    });

    // The ship itself
    c.spawn_bundle(SpaceshipControllerBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    })
    .insert_bundle((
        Player,
        RotateToGoal {
            sensitivity: 1.0,
            ..default()
        },
    ))
    .with_children(|c| {
        c.spawn_bundle(TransformBundle {
            local: Transform::from_translation(Vec3::ZERO).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -FRAC_PI_2,
                0.0,
            )),
            ..default()
        })
        .with_children(|c| {
            c.spawn_scene(ass.load("gltf/starship3.glb#Scene0"));
        });

        c.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 15.0, 30.0)
                .looking_at(vec3(0.0, 5.0, 0.0), Vec3::Y),
            ..default()
        });
    });
}

fn input(
    mut body: Query<(&mut ControllerInput, &GlobalTransform, &mut RotateToGoal)>,
    input: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let (mut body, tf, mut rotate) = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::D) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.forward();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.forward();
    }
    if input.pressed(KeyCode::LControl) {
        dir += -tf.up();
    }
    if input.pressed(KeyCode::Space) {
        dir += tf.up();
    }

    body.movement = dir;

    for &MouseMotion { delta } in mouse.iter() {
        let motion_x = Quat::from_rotation_y(delta.x * time.delta_seconds());
        let motion_y = Quat::from_rotation_x(delta.y * time.delta_seconds());

        rotate.destination = motion_x * motion_y * rotate.destination;
    }
}

fn rotate_to_goal(mut query: Query<(&mut Transform, &RotateToGoal)>) {
    for (mut tf, rotate) in query.iter_mut() {
        tf.rotation = rotate.destination;
    }
}
