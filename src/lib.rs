use bevy::prelude::{App, Events, Plugin, ResMut};
use bevy_egui::EguiContext;
pub use egui_notify::*;

pub mod prelude {
    pub use crate::*;
}

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
