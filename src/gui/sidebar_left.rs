use bevy::prelude::ResMut;
use bevy_egui::{
  egui::{self},
  EguiContexts,
};

use crate::resource::color::Color;

pub fn sidebar_left(mut egui_contexts: EguiContexts, mut color: ResMut<Color>) {
  egui::SidePanel::left("sidebar_left")
    .resizable(false)
    .max_width(55.)
    .show(egui_contexts.ctx_mut(), |ui| {
      let mut new_color: [f32; 4] = [color.r, color.g, color.b, color.a];

      ui.color_edit_button_rgba_unmultiplied(&mut new_color);

      if color.r != new_color[0] {
        color.r = new_color[0];
      }

      if color.g != new_color[1] {
        color.g = new_color[1];
      }

      if color.b != new_color[2] {
        color.b = new_color[2];
      }

      if color.a != new_color[3] {
        color.a = new_color[3];
      }
    });
}
