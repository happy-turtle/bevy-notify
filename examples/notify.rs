use bevy::prelude;
use bevy_notify::NotifyPlugin;

fn main() {
    App::new().add_plugin(NotifyPlugin).run();
}
