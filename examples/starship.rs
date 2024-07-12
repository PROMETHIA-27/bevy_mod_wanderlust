use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::css,
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, CursorGrabMode, PrimaryWindow},
};
use bevy_mod_wanderlust::{
    ControllerBundle, ControllerInput, ControllerPhysicsBundle, WanderlustPlugin,
};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    cursor: Cursor {
                        visible: false,
                        grab_mode: CursorGrabMode::Locked,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WanderlustPlugin,
            aether_spyglass::SpyglassPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input.before(bevy_mod_wanderlust::movement),
                toggle_cursor_lock,
            ),
        )
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
    let mesh = meshes.add(
        Cuboid {
            half_size: Vec3::splat(5.0),
        }
        .into(),
    );
    let material = mats.add(Color::from(css::WHITE));

    c.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Light so you can see the cube
    c.spawn(PointLightBundle {
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
    c.spawn((
        ControllerBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            physics: ControllerPhysicsBundle {
                damping: Damping {
                    angular_damping: 0.5,
                    linear_damping: 0.5,
                },
                ..default()
            },
            ..ControllerBundle::starship()
        },
        Player,
    ))
    .with_children(|c| {
        c.spawn(SceneBundle {
            transform: Transform::from_translation(Vec3::ZERO).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -FRAC_PI_2,
                0.0,
            )),
            scene: ass.load("gltf/starship.glb#Scene0"),
            ..default()
        });

        c.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 7.5, 35.0),
            ..default()
        });
    });
}

fn input(
    mut body: Query<(&mut ControllerInput, &GlobalTransform, &mut ExternalImpulse)>,
    input: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    const SENSITIVITY: f32 = 0.025;
    const ROLL_MULT: f32 = 5.0;

    let (mut body, tf, mut impulse) = body.single_mut();

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
    if input.pressed(KeyCode::ControlLeft) {
        dir += -tf.up();
    }
    if input.pressed(KeyCode::Space) {
        dir += tf.up();
    }

    body.movement = dir;

    let dt = time.delta_seconds();
    for &MouseMotion { delta } in mouse.iter() {
        impulse.torque_impulse += tf.up() * -delta.x * dt * SENSITIVITY;
        impulse.torque_impulse += tf.right() * -delta.y * dt * SENSITIVITY;
    }
    if input.pressed(KeyCode::Q) {
        impulse.torque_impulse += -tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    }
    if input.pressed(KeyCode::E) {
        impulse.torque_impulse += tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    }
}

fn toggle_cursor_lock(
    input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Locked => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}
