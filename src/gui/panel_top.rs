use bevy_egui::{egui, EguiContexts};

pub fn panel_top(mut egui_contexts: EguiContexts) {
    let ctx = egui_contexts.ctx_mut();

    egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            })
        })
    });
}
