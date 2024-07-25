use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::resource::pixel_color::PixelColor;

pub fn left_sidebar(mut contexts: EguiContexts, mut pixel_color_r: ResMut<PixelColor>) {
    egui::SidePanel::left("left_sidebar")
        .resizable(false)
        .max_width(55.)
        .show(contexts.ctx_mut(), |ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut pixel_color_r.value);
        });
}
