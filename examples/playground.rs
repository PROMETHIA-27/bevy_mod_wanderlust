//!

use bevy::render::camera::Projection;
use bevy::window::CursorGrabMode;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};
use bevy_framepace::*;
use bevy_mod_wanderlust::{
    Controller, ControllerBundle, ControllerInput, ControllerPhysicsBundle, GroundCaster, Movement,
    RapierPhysicsBundle, Strength, Upright, WanderlustPlugin, Jump,
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::{FRAC_2_PI, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::At(IVec2::new(0, 0)),
                    resolution: (1000.0, 1080.0).into(),
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
            RapierDebugRenderPlugin::default(),
            WanderlustPlugin::default(),
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            FramepacePlugin,
        ))
        .insert_resource(FramepaceSettings {
            //limiter: Limiter::Manual(std::time::Duration::from_secs_f64(0.1))
            limiter: Limiter::Auto,
        })
        .insert_resource(Sensitivity(1.0))
        .add_systems(
            Startup,
            (player, ground, lights, slopes, moving_objects, steps, walls),
        )
        // Add to PreUpdate to ensure updated before movement is calculated
        .add_systems(
            Update,
            (
                movement_input.before(bevy_mod_wanderlust::movement_force),
                toggle_cursor_lock,
                oscillating,
            ),
        )
        .add_systems(
            PostUpdate,
            mouse_look.before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .run()
}

#[derive(Component)]
struct PlayerCam {
    pub target: Entity,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct PlayerBody;

#[derive(Reflect, Resource)]
struct Sensitivity(f32);

pub fn player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Capsule {
            radius: 0.3,
            depth: 1.0,
            ..default()
        }
        .into(),
    );

    let material = mats.add(Color::WHITE.into());

    let player = commands
        .spawn((
            ControllerBundle {
                rapier_physics: RapierPhysicsBundle {
                    // Lock the axes to prevent camera shake whilst moving up slopes
                    //locked_axes: LockedAxes::ROTATION_LOCKED,
                    ..default()
                },
                controller: Controller {
                    movement: Movement {
                        acceleration_force: Strength::Scaled(5.0),
                        //slip_force_scale: Vec3::splat(0.95),
                        ..default()
                    },
                    jump: Jump {
                        initial_force: 30.0,
                        force: 10.0,
                        ..default()
                    },
                    ground_caster: GroundCaster {
                        cast_collider: Some(Collider::cylinder(0.3, 0.3)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Name::from("Player"),
            PlayerBody,
        ))
        .insert(PbrBundle {
            mesh,
            material: material.clone(),
            ..default()
        })
        .id();

    commands
        .spawn(SpatialBundle::default())
        .insert(PlayerCam {
            target: player,
            pitch: 0.0,
            yaw: 0.0,
        })
        .with_children(|commands| {
            commands
                .spawn((Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.5, 3.0),
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: 90.0 * (PI / 180.0),
                        aspect_ratio: 1.0,
                        near: 0.3,
                        far: 1000.0,
                    }),
                    ..default()
                },))
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
}

pub fn ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(Color::WHITE.into());

    let size = 500.0;
    let mesh = meshes.add(
        shape::Plane {
            size: size,
            ..default()
        }
        .into(),
    );

    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
        //Collider::halfspace(Vec3::Y).unwrap(),
        Collider::cuboid(size / 2.0, 0.1, size / 2.0),
        Name::from("Ground"),
    ));
}

fn lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform {
            rotation: Quat::from_rotation_z(35.0 * PI / 180.0)
                * Quat::from_rotation_y(35.0 * PI / 180.0),
            ..default()
        },
        ..default()
    });
}

pub fn steps(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(StandardMaterial {
        base_color: Color::PINK,
        perceptual_roughness: 0.5,
        reflectance: 0.05,
        ..default()
    });

    let step_increment = 0.2;
    let width = 0.3;
    let steps = 12;
    let stairs = commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(5.0, 0.0, -5.0),
                rotation: Quat::from_rotation_y(PI / 4.0),
                ..default()
            },
            ..default()
        })
        .id();

    for step in 1..=steps {
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: material.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            step as f32 * width,
                            step as f32 * step_increment / 2.0,
                            0.0,
                        ),
                        scale: Vec3::new(width, step as f32 * step_increment, 5.0),
                        ..default()
                    },
                    ..default()
                },
                Name::from("Step"),
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .set_parent(stairs);
    }
}

pub fn walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let materials = [Color::GRAY, Color::WHITE, Color::BLACK];
    let materials = materials
        .iter()
        .map(|color| {
            mats.add(StandardMaterial {
                base_color: *color,
                perceptual_roughness: 0.5,
                reflectance: 0.05,
                ..default()
            })
        })
        .collect::<Vec<_>>();

    let wall = commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(-5.0, 0.0, -5.0),
                rotation: Quat::from_rotation_y(-PI / 4.0),
                ..default()
            },
            ..default()
        })
        .id();

    let parts = 20;
    let width = 0.25;
    for part in 0..=parts {
        let material = materials[part % materials.len()].clone();
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: material,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, part as f32 * width),
                        scale: Vec3::new(1.0, 40.0, width),
                        ..default()
                    },
                    ..default()
                },
                Name::from("Wall segment"),
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .set_parent(wall);
    }

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials[0].clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, ((parts + 1) as f32 * width) * 1.5),
                    scale: Vec3::new(1.0, 40.0, width * parts as f32),
                    ..default()
                },
                ..default()
            },
            Name::from("Full wall segment"),
            Collider::cuboid(0.5, 0.5, 0.5),
        ))
        .set_parent(wall);
}

pub fn slopes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(StandardMaterial {
        base_color: Color::GREEN,
        perceptual_roughness: 0.5,
        reflectance: 0.05,
        ..default()
    });

    let angles = 18;
    let max_angle = PI / 2.0;
    let angle_increment = max_angle / angles as f32;
    for angle in 0..=angles {
        let radians = angle as f32 * angle_increment;
        let width = 2.5;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, angle as f32 * width),
                    rotation: Quat::from_rotation_z(radians),
                    scale: Vec3::new(12.0, 1.0, width),
                },
                ..default()
            },
            Name::from(format!("Slope {:?} radians", radians)),
            Collider::cuboid(0.5, 0.5, 0.5),
        ));
    }
}

pub fn moving_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(StandardMaterial {
        base_color: Color::YELLOW,
        perceptual_roughness: 0.5,
        reflectance: 0.05,
        ..default()
    });
    let mesh = meshes.add(Mesh::from(shape::Cube::default()));

    // simple
    let simple_width = 5.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-5.0, 0.3, 10.0),
                scale: Vec3::new(simple_width, 0.1, simple_width),
                ..default()
            },
            ..default()
        },
        Name::from("Simple moving platform"),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(0.5, 0.5, 0.5),
        Oscillator::default(),
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
    ));
}

#[derive(Component)]
pub struct Oscillator {
    pub strength: Vec3,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            strength: Vec3::ONE,
        }
    }
}

pub fn oscillating(time: Res<Time>, mut oscillators: Query<(&mut Velocity, &Oscillator)>) {
    for (mut velocity, oscillator) in &mut oscillators {
        let elapsed = time.elapsed_seconds();
        let period = 5.0;
        let along = elapsed.rem_euclid(period) / period * std::f32::consts::TAU;
        let x = along.cos();
        let y = along.sin();
        velocity.linvel = Vec3::new(x, 0.0, y) * oscillator.strength;
    }
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
    player_input.movement = dir.normalize_or_zero();

    player_input.jumping = input.pressed(KeyCode::Space);
}

fn mouse_look(
    globals: Query<&GlobalTransform>,
    mut cam: Query<(&mut Transform, &mut PlayerCam)>,
    mut upright: Query<&mut Upright>,
    sensitivity: Res<Sensitivity>,
    mut input: EventReader<MouseMotion>,
) {
    let (mut cam_tf, mut player_cam) = cam.single_mut();
    let target_global = globals.get(player_cam.target).unwrap();
    cam_tf.translation = target_global.translation();

    let mut upright = upright.single_mut();

    let sens = sensitivity.0;
    let cumulative: Vec2 = -(input.iter().map(|motion| &motion.delta).sum::<Vec2>());
    player_cam.pitch += cumulative.y as f32 / 180.0 * sens;
    player_cam.yaw += cumulative.x as f32 / 180.0 * sens;
    // Ensure the vertical rotation is clamped
    let pitch_limit = PI / 2.0 + (3.0 * PI / 180.0);
    player_cam.pitch = player_cam.pitch.clamp(-pitch_limit, pitch_limit);

    cam_tf.rotation =
        Quat::from_rotation_y(player_cam.yaw) * Quat::from_rotation_x(player_cam.pitch);
    upright.forward_vector = Some(Quat::from_rotation_y(player_cam.yaw) * Vec3::X);
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
