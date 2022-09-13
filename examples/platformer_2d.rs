//! A simple example of setting up a 2D platformer character controlled player.

use bevy::prelude::*;
use bevy_mod_wanderlust::backends::Rapier2dBackend;
use bevy_mod_wanderlust::{CharacterControllerBundle, ControllerInput, WanderlustPlugin};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WanderlustPlugin(Rapier2dBackend))
        .insert_resource(Sensitivity(0.15))
        .add_startup_system(setup)
        // Add to PreUpdate to ensure updated before movement is calculated
        .add_system_to_stage(CoreStage::PreUpdate, movement_input)
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

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0).with_scale(Vec3::new(0.03, 0.02, 1.0)),
        ..default()
    });

    commands
        .spawn_bundle(CharacterControllerBundle::<Rapier2dBackend>::default())
        .insert_bundle(SpatialBundle::default())
        .insert_bundle((Name::from("Player"), PlayerBody));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, -3.0, 0.0),
            ..default()
        })
        .insert_bundle((Collider::cuboid(10.0, 0.5), Name::from("Ground")));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(-6.5, -1.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                0.0,
                0.9,
            )),
            ..default()
        })
        .insert_bundle((Name::from("Slope"), Collider::cuboid(0.5, 3.0)));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(6.5, -1.0, 0.0),
            ..default()
        })
        .insert_bundle((Name::from("Wall"), Collider::cuboid(0.25, 3.0)));
}

fn movement_input(
    mut body: Query<&mut ControllerInput, With<PlayerBody>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player_input = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::Left) {
        dir += -Vec3::X;
    }
    if input.pressed(KeyCode::Right) {
        dir += Vec3::X
    }
    dir.y = 0.0;
    player_input.movement = dir;

    player_input.jumping = input.pressed(KeyCode::Up);
}
