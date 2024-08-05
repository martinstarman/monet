use bevy::prelude::ResMut;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::resource::color::Color;

pub fn left_sidebar(mut egui_contexts: EguiContexts, mut pixel_color_r: ResMut<Color>) {
    egui::SidePanel::left("left_sidebar")
        .resizable(false)
        .max_width(55.)
        .show(egui_contexts.ctx_mut(), |ui| {
            let mut color: [f32; 4] = [
                pixel_color_r.r,
                pixel_color_r.g,
                pixel_color_r.b,
                pixel_color_r.a,
            ];

            ui.color_edit_button_rgba_unmultiplied(&mut color);

            if pixel_color_r.r != color[0] {
                pixel_color_r.r = color[0];
            }

            if pixel_color_r.g != color[1] {
                pixel_color_r.g = color[1];
            }

            if pixel_color_r.b != color[2] {
                pixel_color_r.b = color[2];
            }

            if pixel_color_r.a != color[3] {
                pixel_color_r.a = color[3];
            }
        });
}
