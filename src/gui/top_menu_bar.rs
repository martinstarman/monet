use bevy_egui::{egui, EguiContexts};

pub fn top_menu_bar(mut contexts: EguiContexts) {
    egui::TopBottomPanel::top("top_menu_bar").show(contexts.ctx_mut(), |_ui| {
        // TODO:
    });
}
