use bevy::prelude::{EventWriter, Query};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::{component::layer::Layer, event::new_layer::NewLayer};

pub fn right_sidebar(
    mut egui_contexts: EguiContexts,
    mut layers_q: Query<&mut Layer>,
    mut event_writer: EventWriter<NewLayer>,
) {
    egui::SidePanel::right("right_sidebar")
        .resizable(false)
        .min_width(100.)
        .max_width(100.) // TODO
        .show(egui_contexts.ctx_mut(), |ui| {
            if ui.button("new layer").clicked() {
                event_writer.send(NewLayer {});
            }

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    for mut layer in &mut layers_q {
                        ui.text_edit_singleline(&mut layer.name);
                    }
                })
        });
}
