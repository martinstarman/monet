use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::resource::image_dimensions::ImageDimensions;

pub fn panel_bottom(
  mut egui_contexts: EguiContexts,
  mut image_dimensions: ResMut<ImageDimensions>,
) {
  egui::TopBottomPanel::bottom("panel_bottom").show(egui_contexts.ctx_mut(), |ui| {
    let mut width = image_dimensions.width;
    let mut height = image_dimensions.height;

    egui::Grid::new("dimensions").show(ui, |ui| {
      ui.add(egui::DragValue::new(&mut width).speed(1.0).max_decimals(0));
      ui.label("×");
      ui.add(egui::DragValue::new(&mut height).speed(1.0).max_decimals(0));
    });

    if width > 0 && width != image_dimensions.width {
      image_dimensions.width = width;
    }

    if height > 0 && height != image_dimensions.height {
      image_dimensions.height = height;
    }
  });
}
