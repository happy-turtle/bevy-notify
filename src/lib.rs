use bevy::prelude::{App, Events, Plugin, ResMut};
use bevy_egui::EguiContext;
pub use egui_notify::ToastLevel;
use egui_notify::*;

pub mod prelude {
    pub use crate::*;
}

pub struct NotifyPlugin;
pub struct ToastEvent {
    pub label: String,
    pub duration: std::time::Duration,
    pub level: ToastLevel,
    pub closable: bool,
}

impl Plugin for NotifyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Toasts::default())
            .insert_resource(Events::<ToastEvent>::default())
            .add_system(update_toasts);
    }
}

fn update_toasts(
    mut egui_context: ResMut<EguiContext>,
    mut toasts: ResMut<Toasts>,
    mut events: ResMut<Events<ToastEvent>>,
) {
    events.update();
    for event in events.drain() {
        toasts
            .info(event.label.as_str())
            .set_duration(Some(event.duration))
            .set_level(event.level)
            .set_closable(event.closable);
    }
    toasts.show(egui_context.ctx_mut());
}
