use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_notify::prelude::*;

fn main() {
    let mut app = App::new();
    // everything you need for bevy-notify
    app.add_plugin(NotifyPlugin)
        .add_plugin(EguiPlugin)
        .insert_resource(Toasts::default());

    // sample app
    app.add_plugins(DefaultPlugins)
        .add_startup_system(scene_setup)
        .add_system(notify_example)
        .run();
}

fn notify_example(key_input: Res<Input<KeyCode>>, mut events: ResMut<Events<Toast>>) {
    if key_input.just_pressed(KeyCode::Space) {
        events.send(Toast::success("Space pressed"));
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
}
