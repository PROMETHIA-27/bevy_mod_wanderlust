//! A simple example of setting up a first-person character controlled player.

use bevy::render::camera::Projection;
use bevy::window::CursorGrabMode;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};
use bevy_mod_wanderlust::{ControllerBundle, ControllerInput, WanderlustPlugin};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: {
                    let mut cursor = Cursor::default();
                    cursor.visible = false;
                    cursor.grab_mode = CursorGrabMode::Locked;
                    cursor
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WanderlustPlugin)
        .insert_resource(Sensitivity(1.0))
        .add_startup_system(setup)
        // Add to PreUpdate to ensure updated before movement is calculated
        .add_system(movement_input.before(bevy_mod_wanderlust::movement))
        .add_system(mouse_look)
        .add_system(toggle_cursor_lock)
        //.add_plugin(EditorPlugin)
        .run()
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct PlayerCam;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct PlayerBody;

#[derive(Reflect, Resource)]
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
        .spawn((
            ControllerBundle::character(),
            Name::from("Player"),
            PlayerBody,
        ))
        .insert(PbrBundle {
            mesh,
            material: material.clone(),
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn((
                    Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: 90.0 * (std::f32::consts::PI / 180.0),
                            aspect_ratio: 1.0,
                            near: 0.3,
                            far: 1000.0,
                        }),
                        ..default()
                    },
                    PlayerCam,
                ))
                .with_children(|commands| {
                    let mesh = meshes.add(shape::Cube { size: 0.5 }.into());

                    commands.spawn(PbrBundle {
                        mesh,
                        material: material.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -0.5),
                        ..default()
                    });
                });
        });

    let mesh = meshes.add(
        shape::Plane {
            size: 10.0,
            ..default()
        }
        .into(),
    );

    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, -10.0, 0.0),
            ..default()
        },
        Collider::halfspace(Vec3::Y * 10.0).unwrap(),
        Name::from("Ground"),
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });

    let (hw, hh, hl) = (1.5, 0.5, 5.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(-3.5, -8.0, 0.3).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.5,
                0.0,
                0.0,
            )),
            ..default()
        },
        Name::from("Slope"),
        Collider::cuboid(hw, hh, hl),
    ));

    let (hw, hh, hl) = (0.25, 3.0, 5.0);
    let mesh = meshes.add(
        shape::Box {
            min_x: -hw,
            max_x: hw,
            min_y: -hh,
            max_y: hh,
            min_z: -hl,
            max_z: hl,
        }
        .into(),
    );

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(3.5, -8.0, 0.0),
            ..default()
        },
        Name::from("Wall"),
        Collider::cuboid(hw, hh, hl),
    ));

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(6.5, -8.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -std::f32::consts::FRAC_PI_4,
                0.0,
            )),
            ..default()
        },
        Name::from("Wall"),
        Collider::cuboid(hw, hh, hl),
    ));
}

fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    camera: Query<&GlobalTransform, (With<PlayerCam>, Without<PlayerBody>)>,
    input: Res<Input<KeyCode>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

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
    dir.y = 0.0;
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = input.pressed(KeyCode::Space);
}

fn mouse_look(
    mut cam: Query<&mut Transform, With<PlayerCam>>,
    mut body: Query<&mut Transform, (With<PlayerBody>, Without<PlayerCam>)>,
    sensitivity: Res<Sensitivity>,
    mut input: EventReader<MouseMotion>,
) {
    let mut cam_tf = cam.single_mut();
    let mut body_tf = body.single_mut();

    let sens = sensitivity.0;

    let cumulative: Vec2 = input.iter().map(|motion| &motion.delta).sum();

    // Vertical
    let rot = cam_tf.rotation;
    cam_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::X * (-cumulative.y / 180.0) * sens,
    ));

    // Horizontal
    let rot = body_tf.rotation;
    body_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::Y * (-cumulative.x / 180.0) * sens,
    ));
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
