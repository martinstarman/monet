use bevy::prelude::ResMut;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::resource::color::Color;

pub fn left_sidebar(mut egui_contexts: EguiContexts, mut pixel_color_r: ResMut<Color>) {
    egui::SidePanel::left("left_sidebar")
        .resizable(false)
        .max_width(55.) // TODO
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut pixel_color_r.value);
        });
}
