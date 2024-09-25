use bevy::prelude::{Entity, EventWriter, Query, Visibility};
use bevy_egui::{
  egui::{self},
  EguiContexts,
};

use crate::{
  component::layer::Layer,
  event::{add_layer::AddLayer, change_active_layer::ChangeActiveLayer},
};

pub fn sidebar_right(
  mut egui_contexts: EguiContexts,
  mut layers: Query<(Entity, &mut Layer, &mut Visibility)>,
  mut add_layer_event_writer: EventWriter<AddLayer>,
  mut set_active_layer_event_writer: EventWriter<ChangeActiveLayer>,
) {
  egui::SidePanel::right("sidebar_right")
    .resizable(false)
    .min_width(100.)
    .max_width(100.)
    .show(egui_contexts.ctx_mut(), |ui| {
      if ui.button("new layer").clicked() {
        add_layer_event_writer.send(AddLayer {});
      }

      egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
          for (entity, mut layer, mut visibility) in &mut layers {
            if ui
              .add(egui::RadioButton::new(layer.is_active, "is_active"))
              .clicked()
            {
              set_active_layer_event_writer.send(ChangeActiveLayer { id: entity });
            }

            // TODO: event
            if ui
              .add(egui::Checkbox::new(&mut layer.is_visible, "is_visible"))
              .clicked()
            {
              *visibility = if layer.is_visible {
                Visibility::Visible
              } else {
                Visibility::Hidden
              };
            }

            ui.text_edit_singleline(&mut layer.name);
          }
        })
    });
}
