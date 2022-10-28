use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_notify::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NotifyPlugin)
        .add_plugin(EguiPlugin)
        .add_system(notify_example)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_startup_system(scene_setup)
        .run();
}

fn notify_example(mouse_input: Res<Input<MouseButton>>, mut events: ResMut<Events<ToastEvent>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // TODO: default fields
        events.send(ToastEvent {
            label: "Hello world".to_string(),
            duration: Duration::from_secs(5),
            level: ToastLevel::Error,
            closable: true,
        });
    }
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            ..default()
        })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.3).into()),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}
