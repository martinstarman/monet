use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::resource::image_dimension::ImageDimension;

pub fn bottom_bar(mut egui_contexts: EguiContexts, mut image_dimension_r: ResMut<ImageDimension>) {
    egui::TopBottomPanel::bottom("bottom_bar").show(egui_contexts.ctx_mut(), |ui| {
        let mut width = image_dimension_r.width;
        let mut height = image_dimension_r.height;

        egui::Grid::new("dimensions").show(ui, |ui| {
            ui.add(egui::DragValue::new(&mut width).speed(1.0).max_decimals(0));

            ui.label("×");

            ui.add(egui::DragValue::new(&mut height).speed(1.0).max_decimals(0));
        });

        if width > 0 && width != image_dimension_r.width {
            image_dimension_r.width = width;
        }

        if height > 0 && height != image_dimension_r.height {
            image_dimension_r.height = height;
        }
    });
}
