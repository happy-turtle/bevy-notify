//! # bevy-notify
//!
//! `bevy-notify` is a bevy plugin wrapping [egui_notify](https://crates.io/crates/egui-notify) and
//! adding an event receiver to your bevy app. Toast notifications can then be send
//! in your bevy app at any point.
//! # Examples
//!
//! ```
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(NotifyPlugin)
//!         .add_plugin(EguiPlugin)
//!         .insert_resource(Toasts::default())
//!         .add_system(notify_example_system)
//!         .run();
//! }
//!    
//! fn notify_example_system(key_input: Res<Input<KeyCode>>, mut events: ResMut<Events<Toast>>) {
//!     if key_input.just_pressed(KeyCode::Space) {
//!         events.send(Toast::success("Space pressed"));
//!     }
//! }
//! ```
use bevy::prelude::{App, Events, Plugin, ResMut};
use bevy_egui::EguiContext;
pub use egui_notify::*;

/// Adds an event resource for Toast nofications to your bevy app.
/// A system is added that will show received toast events through an egui context.
///
/// # Examples
///
/// ```
/// App::new().add_plugin(NotifyPlugin)
///     .add_plugin(EguiPlugin)
///     .insert_resource(Toasts::default());
/// ```
pub struct NotifyPlugin;

impl Plugin for NotifyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Events::<Toast>::default())
            .add_system(update_toasts);
    }
}

fn update_toasts(
    mut egui_context: ResMut<EguiContext>,
    mut toasts: ResMut<Toasts>,
    mut events: ResMut<Events<Toast>>,
) {
    events.update();
    for event in events.drain() {
        toasts.add(event);
    }
    toasts.show(egui_context.ctx_mut());
}
