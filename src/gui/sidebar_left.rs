use bevy::prelude::ResMut;
use bevy_egui::{
  egui::{self},
  EguiContexts,
};

use crate::resource::color::Color;

pub fn sidebar_left(mut egui_contexts: EguiContexts, mut color_r: ResMut<Color>) {
  egui::SidePanel::left("left_sidebar")
    .resizable(false)
    .max_width(55.)
    .show(egui_contexts.ctx_mut(), |ui| {
      let mut color: [f32; 4] = [color_r.r, color_r.g, color_r.b, color_r.a];

      ui.color_edit_button_rgba_unmultiplied(&mut color);

      if color_r.r != color[0] {
        color_r.r = color[0];
      }

      if color_r.g != color[1] {
        color_r.g = color[1];
      }

      if color_r.b != color[2] {
        color_r.b = color[2];
      }

      if color_r.a != color[3] {
        color_r.a = color[3];
      }
    });
}
