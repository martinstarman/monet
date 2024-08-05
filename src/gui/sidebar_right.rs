use bevy::prelude::{EventWriter, Query, Visibility};
use bevy_egui::{
  egui::{self},
  EguiContexts,
};

use crate::{
  component::layer::Layer,
  event::{add_layer::AddLayer, set_active_layer::SetActiveLayer},
};

pub fn sidebar_right(
  mut egui_contexts: EguiContexts,
  mut layers_q: Query<(&mut Layer, &mut Visibility)>,
  mut new_layer_event: EventWriter<AddLayer>,
  mut set_active_layer_event: EventWriter<SetActiveLayer>,
) {
  egui::SidePanel::right("right_sidebar")
    .resizable(false)
    .min_width(100.)
    .max_width(100.)
    .show(egui_contexts.ctx_mut(), |ui| {
      if ui.button("new layer").clicked() {
        new_layer_event.send(AddLayer {
          name: None,
          index: None,
          active: Some(true),
          visible: Some(true),
        });
      }

      egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
          for (mut layer, mut visibility) in &mut layers_q {
            if ui.add(egui::RadioButton::new(layer.active, "")).clicked() {
              set_active_layer_event.send(SetActiveLayer {
                layer_index: layer.index,
              });
            }

            // TODO: system
            if ui
              .add(egui::Checkbox::new(&mut layer.visible, "visible"))
              .clicked()
            {
              *visibility = if layer.visible {
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
