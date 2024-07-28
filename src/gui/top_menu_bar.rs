use bevy_egui::{egui, EguiContexts};

pub fn top_menu_bar(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

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
