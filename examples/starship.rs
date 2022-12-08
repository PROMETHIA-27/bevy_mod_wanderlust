use std::f32::consts::FRAC_PI_2;

// use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
// use bevy_editor_pls::EditorPlugin;
// use bevy_editor_pls::controls::{EditorControls, Action, Binding, UserInput};
// use bevy_editor_pls::controls::{Action, Binding, Button, EditorControls, UserInput};
// use bevy_editor_pls::prelude::*;
use bevy_mod_wanderlust::{
    ControllerInput, ControllerPhysicsBundle, StarshipControllerBundle, WanderlustPlugin,
};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::Damping;

fn main() {
    let mut bindings = EditorControls::default_bindings();
    bindings.unbind(Action::PlayPauseEditor);
    bindings.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Chord(vec![
                Button::Keyboard(KeyCode::LControl),
                Button::Keyboard(KeyCode::E),
            ]),
            conditions: vec![],
        },
    );

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WanderlustPlugin)
        .add_plugin(EditorPlugin)
        .insert_resource(bindings)
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

    c.spawn(PbrBundle {
        mesh,
        material: mat.clone(),
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
    c.spawn(StarshipControllerBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        physics: ControllerPhysicsBundle {
            damping: Damping {
                angular_damping: 0.5,
                linear_damping: 0.5,
            },
            ..default()
        },
        ..default()
    })
    .insert((Player,))
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
    mut body: Query<(&mut ControllerInput, &GlobalTransform)>,
    input: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    const SENSITIVITY: f32 = 0.025;
    const ROLL_MULT: f32 = 5.0;

    let (mut body, tf) = body.single_mut();

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

    let dt = time.delta_seconds();
    for &MouseMotion { delta } in mouse.iter() {
        body.torque += tf.up() * -delta.x * dt * SENSITIVITY;
        body.torque_impulse += tf.right() * -delta.y * dt * SENSITIVITY;
    }
    if input.pressed(KeyCode::Q) {
        body.torque_impulse += -tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    }
    if input.pressed(KeyCode::E) {
        body.torque_impulse += tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    }
}
