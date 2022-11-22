# Bevy Notify

A bevy plugin wrapping the crate [`egui-notify`](https://github.com/ItsEthra/egui-notify) to allow sending toast messages utilizing events.

## Usage

```
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NotifyPlugin)
        .add_plugin(EguiPlugin)
        .insert_resource(Notifications(Toasts::default()))
        .add_system(notify_example_system)
        .run();
}

fn notify_example_system(key_input: Res<Input<KeyCode>>, mut events: ResMut<Events<Toast>>) {
    if key_input.just_pressed(KeyCode::Space) {
        events.send(Toast::success("Space pressed"));
    }
}
```

## Setup

Add the plugin to your bevy app alongside the `EguiPlugin` from [bevy_egui](https://crates.io/crates/bevy_egui). You also need to add a `Toasts` resource to your app.

```
App::new().add_plugin(NotifyPlugin)
    .add_plugin(EguiPlugin)
    .insert_resource(Notifications(Toasts::default()));
```
