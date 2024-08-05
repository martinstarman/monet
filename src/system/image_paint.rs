use bevy::prelude::{Camera, Camera2d, EventWriter, GlobalTransform, Query, Window, With};
use bevy_egui::{egui, EguiContexts};

use crate::event::paint_at::PaintAt;

pub fn image_paint(
    mut paint_at_event_writer: EventWriter<PaintAt>,
    windows_q: Query<&Window>,
    camera_q: Query<(&mut Camera, &mut GlobalTransform), With<Camera2d>>,
    mut egui_contexts: EguiContexts,
) {
    if egui_contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    egui_contexts.ctx_mut().input(|input| {
        if input.pointer.button_down(egui::PointerButton::Primary) {
            let window = windows_q.single();
            let cursor_position = window.cursor_position();

            if cursor_position.is_none() {
                return;
            }

            let (camera, global_transform) = camera_q.single();
            let position = camera.viewport_to_world_2d(global_transform, cursor_position.unwrap());

            if position.is_none() {
                return;
            }

            paint_at_event_writer.send(PaintAt {
                x: position.unwrap().x,
                y: position.unwrap().y,
            });
        }
    });
}
