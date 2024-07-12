//!

use bevy::render::camera::Projection;
use bevy::window::CursorGrabMode;
use bevy::{
    color::palettes::css,
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};
use bevy_framepace::*;
use bevy_mod_wanderlust::{
    Controller, ControllerBundle, ControllerInput, ControllerPhysicsBundle, Float, GroundCaster,
    Jump, Movement, RapierPhysicsBundle, Spring, SpringStrength, Strength, Upright,
    WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::{FRAC_2_PI, PI};

fn main() -> AppExit {
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
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 0.016,
                substeps: 32,
            },
            ..RapierConfiguration::new(1.0)
        })
        .insert_resource(FramepaceSettings {
            limiter: Limiter::Manual(std::time::Duration::from_secs_f64(0.016)),
            //limiter: Limiter::Manual(std::time::Duration::from_secs_f64(0.1)),
        })
        .insert_resource(Sensitivity(1.0))
        .add_systems(
            Startup,
            (
                player,
                ground,
                lights,
                slopes,
                moving_objects,
                stairs,
                walls,
                free_objects,
            ),
        )
        .add_systems(
            Last,
            |input: Res<ButtonInput<KeyCode>>,
             //mut freeze: ResMut<Freeze>,
             mut freeze: Local<bool>,
             mut time: ResMut<Time>,
             mut rapier_config: ResMut<RapierConfiguration>| {
                if input.just_pressed(KeyCode::KeyR) {
                    *freeze = !*freeze;
                }

                if !*freeze || input.just_pressed(KeyCode::KeyF) {
                    rapier_config.timestep_mode = TimestepMode::Fixed {
                        dt: 0.016,
                        substeps: 32,
                    };
                } else {
                    *time = Time::default();
                    rapier_config.timestep_mode = TimestepMode::Fixed {
                        dt: 0.0,
                        substeps: 1,
                    };
                }
            },
        )
        // Add to PreUpdate to ensure updated before movement is calculated
        .add_systems(
            Update,
            (
                movement_input.before(bevy_mod_wanderlust::movement_force),
                toggle_cursor_lock,
                oscillating,
                controlled_platform,
            ),
        )
        .add_systems(
            PostUpdate,
            mouse_look.before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_systems(
            Update,
            |input: Res<ButtonInput<KeyCode>>, mut impulses: Query<&mut ExternalImpulse>| {
                if !input.just_pressed(KeyCode::KeyP) {
                    return;
                }

                for mut impulse in &mut impulses {
                    impulse.impulse += Vec3::new(1.0, 0.0, 0.0) * 10.0;
                }
            },
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
    let mesh = meshes.add(Capsule3d {
        radius: 0.3,
        half_length: 0.5,
    });

    let material = mats.add(Color::from(css::WHITE));

    let player = commands
        .spawn((
            ControllerBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 3.0, 0.0),
                    ..default()
                },
                rapier_physics: RapierPhysicsBundle {
                    // Lock the axes to prevent camera shake whilst moving up slopes
                    //locked_axes: LockedAxes::ROTATION_LOCKED,
                    //locked_axes: LockedAxes::all(),
                    ..default()
                },
                controller: Controller {
                    movement: Movement {
                        acceleration: Strength::Scaled(5.0),
                        max_speed: 5.0,
                        //slip_force_scale: Vec3::splat(0.95),
                        ..default()
                    },
                    jump: Jump {
                        initial_force: 30.0,
                        force: 20.0,
                        ..default()
                    },
                    float: Float {
                        distance: 1.0,
                        ..default()
                    },
                    upright: Upright {
                        spring: Spring {
                            strength: SpringStrength::AngularFrequency(20.0),
                            damping: 2.0,
                            ..default()
                        },
                        ..default()
                    },
                    ground_caster: GroundCaster {
                        //cast_collider: Some(Collider::cylinder(0.3, 0.3)),
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
            //mesh,
            material: material.clone(),
            ..default()
        })
        .id();

    let origin_dummy = commands.spawn(SpatialBundle::default()).id();
    commands
        .spawn(SpatialBundle::default())
        .insert(PlayerCam {
            target: player,
            //target: origin_dummy,
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
}

pub fn free_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(Color::from(css::WHITE));
    let mesh = meshes.add(Sphere { radius: 0.5 });
    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(-2.0, 2.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::default(),
        ExternalImpulse::default(),
        ReadMassProperties::default(),
        Collider::ball(0.5),
        Name::from("Ball"),
    ));
}

pub fn ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(Color::from(css::BLACK));

    let size = 1000.0;
    let mesh = meshes.add(Plane3d {
        half_size: Vec2::splat(size / 2.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
        Collider::halfspace(Vec3::Y).unwrap(),
        ColliderDebugColor(
            Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            }
            .into(),
        ),
        //Collider::cuboid(size / 2.0, 0.1, size / 2.0),
        Name::from("Ground"),
    ));

    let material = mats.add(Color::from(css::WHITE));
    let mesh = meshes.add(Sphere::new(0.5));
    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(-2.5, 0.5, 2.0),
            ..default()
        },
        RigidBody::Fixed,
        Velocity::default(),
        ExternalImpulse::default(),
        ReadMassProperties::default(),
        Collider::ball(0.5),
        Name::from("Ball"),
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

pub fn stairs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(StandardMaterial {
        base_color: Color::from(css::PINK),
        perceptual_roughness: 0.5,
        reflectance: 0.05,
        ..default()
    });

    let mesh = meshes.add(Mesh::from(Cuboid {
        half_size: Vec3::splat(0.5),
    }));
    steps(
        Transform {
            translation: Vec3::new(5.0, 0.0, -5.0),
            rotation: Quat::from_rotation_y(PI / 4.0),
            ..default()
        },
        0.2,
        0.3,
        12,
        &mut commands,
        &material,
        &mesh,
    );

    steps(
        Transform {
            translation: Vec3::new(1.0, 0.0, -9.0),
            rotation: Quat::from_rotation_y(PI / 4.0),
            ..default()
        },
        0.4,
        0.3,
        36,
        &mut commands,
        &material,
        &mesh,
    );
}

pub fn steps(
    transform: Transform,
    step_increment: f32,
    width: f32,
    steps: u32,
    commands: &mut Commands,
    material: &Handle<StandardMaterial>,
    mesh: &Handle<Mesh>,
) {
    let stairs = commands
        .spawn(SpatialBundle {
            transform: transform,
            ..default()
        })
        .id();

    for step in 1..=steps {
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
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
    let materials = [css::GRAY, css::WHITE, css::BLACK];
    let materials = materials
        .iter()
        .map(|color| {
            mats.add(StandardMaterial {
                base_color: (*color).into(),
                perceptual_roughness: 0.5,
                reflectance: 0.05,
                ..default()
            })
        })
        .collect::<Vec<_>>();

    let wall = commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(-2.0, 0.0, -2.0),
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
                    mesh: meshes.add(Mesh::from(Cuboid {
                        half_size: Vec3::splat(0.5),
                    })),
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
                mesh: meshes.add(Mesh::from(Cuboid {
                    half_size: Vec3::splat(0.5),
                })),
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
        base_color: css::GREEN.into(),
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

        let rotation_diff = width * radians;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {
                    half_size: Vec3::splat(0.5),
                })),
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(rotation_diff, 0.0, angle as f32 * width + width * 2.0),
                    rotation: Quat::from_rotation_z(radians),
                    scale: Vec3::new(12.0, 1.0, width),
                },
                ..default()
            },
            Name::from(format!("Slope {:?} radians", radians)),
            Collider::cuboid(0.5, 0.5, 0.5),
        ));

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {
                    half_size: Vec3::splat(0.5),
                })),
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        12.0 - rotation_diff,
                        0.0,
                        angle as f32 * width + width * 2.0,
                    ),
                    rotation: Quat::from_rotation_z(-radians),
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
        base_color: css::YELLOW.into(),
        perceptual_roughness: 0.5,
        reflectance: 0.05,
        ..default()
    });
    let mesh = meshes.add(Mesh::from(Cuboid::default()));

    // simple
    let simple_width = 3.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {
                half_size: Vec3::splat(0.5),
            })),
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

    // rotating
    let simple_width = 3.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::default())),
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-10.0, 0.3, 10.0),
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
            angvel: Vec3::new(0.0, 0.1, 0.0),
        },
    ));

    // controlled

    let simple_width = 3.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {
                half_size: Vec3::splat(0.5),
            })),
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(-5.0, 0.3, 10.0),
                scale: Vec3::new(simple_width, 0.1, simple_width),
                ..default()
            },
            ..default()
        },
        Name::from("Controlled moving platform"),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(0.5, 0.5, 0.5),
        Controlled,
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
    ));
}

#[derive(Component)]
pub struct Controlled;

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

pub fn controlled_platform(
    input: Res<ButtonInput<KeyCode>>,
    mut controlled: Query<(&mut Velocity, &Controlled)>,
) {
    for (mut velocity, _) in &mut controlled {
        if input.pressed(KeyCode::KeyH) {
            velocity.linvel.x = 0.0;
        }

        if input.pressed(KeyCode::KeyN) {
            velocity.linvel.x = 5.0;
        }

        if input.pressed(KeyCode::KeyM) {
            velocity.linvel.x = -5.0;
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
    mut body: Query<(&mut ControllerInput, &mut Movement), With<PlayerBody>>,
    camera: Query<&PlayerCam>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let camera = camera.single();
    let camera_dir = Quat::from_rotation_y(camera.yaw);
    let right = camera_dir * Vec3::X;
    let forward = camera_dir * -Vec3::Z;

    let (mut player_input, mut movement) = body.single_mut();

    if input.pressed(KeyCode::ShiftLeft) {
        movement.max_speed = 15.0;
    } else {
        movement.max_speed = 5.0;
    }

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        dir += -right;
    }
    if input.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if input.pressed(KeyCode::KeyS) {
        dir += -forward;
    }
    if input.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    dir.y = 0.0;
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
    let cumulative: Vec2 = -(input.read().map(|motion| &motion.delta).sum::<Vec2>());
    player_cam.pitch += cumulative.y as f32 / 180.0 * sens;
    player_cam.yaw += cumulative.x as f32 / 180.0 * sens;

    // Ensure the vertical rotation is clamped
    let pitch_limit = PI / 2.0;
    player_cam.pitch = player_cam
        .pitch
        .clamp(-pitch_limit + 5.0 * PI / 180.0, pitch_limit);

    cam_tf.rotation =
        Quat::from_rotation_y(player_cam.yaw) * Quat::from_rotation_x(player_cam.pitch);
    upright.forward_vector = Some(Quat::from_rotation_y(player_cam.yaw) * Vec3::X);
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
