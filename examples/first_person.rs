//! A simple example of setting up a first-person character controlled player.

use bevy::render::camera::Projection;
use bevy::window::CursorGrabMode;
use bevy::{
    color::palettes::css,
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_mod_wanderlust::{
    ControllerBundle, ControllerInput, ControllerPhysicsBundle, RapierPhysicsBundle,
    WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_2_PI;

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
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            // This plugin was causing unhelpful glitchy orange planes, so it's commented out until
            // it's working again
            RapierDebugRenderPlugin::default(),
            WanderlustPlugin::default(),
            FramepacePlugin,
        ))
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 0.008,
                substeps: 4,
            },
            ..RapierConfiguration::new(1.0)
        })
        .insert_resource(FramepaceSettings {
            limiter: Limiter::Manual(std::time::Duration::from_secs_f64(0.008)),
        })
        .insert_resource(Sensitivity(1.0))
        .add_systems(Startup, setup)
        // Add to PreUpdate to ensure updated before movement is calculated
        .add_systems(
            Update,
            (
                movement_input.before(bevy_mod_wanderlust::movement_force),
                mouse_look,
                toggle_cursor_lock,
            ),
        )
        .run();
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
    let mesh = meshes.add(Capsule3d {
        radius: 0.5,
        half_length: 0.5,
        ..default()
    });

    let material = mats.add(Color::from(css::WHITE));

    commands
        .spawn((
            ControllerBundle {
                rapier_physics: RapierPhysicsBundle {
                    // Lock the axes to prevent camera shake whilst moving up slopes
                    locked_axes: LockedAxes::ROTATION_LOCKED,
                    restitution: Restitution {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    ..default()
                },
                ..default()
            },
            ColliderMassProperties::Density(50.0),
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
                            near: 0.1,
                            far: 1000.0,
                        }),
                        ..default()
                    },
                    PlayerCam,
                ))
                .with_children(|commands| {
                    let mesh = meshes.add(Cuboid {
                        half_size: Vec3::splat(0.25),
                    });

                    commands.spawn(PbrBundle {
                        mesh,
                        material: material.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -0.5),
                        ..default()
                    });
                });
        });

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)));

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
    let min = Vec3::new(-hw, -hh, -hl);
    let max = Vec3::new(hw, hh, hl);
    let mesh = meshes.add(Cuboid::from_corners(min, max));

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
    let min = Vec3::new(-hw, -hh, -hl);
    let max = Vec3::new(hw, hh, hl);
    let mesh = meshes.add(Cuboid::from_corners(min, max));

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
    input: Res<ButtonInput<KeyCode>>,
) {
    let tf = camera.single();

    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        dir += -tf.right().as_vec3();
    }
    if input.pressed(KeyCode::KeyD) {
        dir += tf.right().as_vec3();
    }
    if input.pressed(KeyCode::KeyS) {
        dir += -tf.forward().as_vec3();
    }
    if input.pressed(KeyCode::KeyW) {
        dir += tf.forward().as_vec3();
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

    let mut cumulative: Vec2 = -(input.read().map(|motion| &motion.delta).sum::<Vec2>());

    // Vertical
    let rot = cam_tf.rotation;

    // Ensure the vertical rotation is clamped
    if rot.x > FRAC_2_PI && cumulative.y.is_sign_positive()
        || rot.x < -FRAC_2_PI && cumulative.y.is_sign_negative()
    {
        cumulative.y = 0.0;
    }

    cam_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::X * cumulative.y / 180.0 * sens,
    ));

    // Horizontal
    let rot = body_tf.rotation;
    body_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::Y * cumulative.x / 180.0 * sens,
    ));
}

fn toggle_cursor_lock(
    input: Res<ButtonInput<KeyCode>>,
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
