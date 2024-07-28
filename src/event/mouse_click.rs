use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_egui::EguiContexts;

use super::draw_pixel::DrawPixel;

pub fn mouse_click(
    mut event_reader: EventReader<MouseButtonInput>,
    mut event_writer: EventWriter<DrawPixel>,
    windows_q: Query<&Window>,
    mut camera_q: Query<
        (
            &mut Camera,
            &mut GlobalTransform,
            &mut OrthographicProjection,
            &mut Transform,
        ),
        With<Camera2d>,
    >,
    mut contexts: EguiContexts,
) {
    for event in event_reader.read() {
        if contexts.ctx_mut().is_pointer_over_area() {
            return;
        }

        if event.button == MouseButton::Left {
            let window = windows_q.single();
            let cursor_position = window.cursor_position();

            if cursor_position.is_none() {
                return;
            }

            let (camera, global_transform, _, _) = camera_q.single();
            let position = camera.viewport_to_world_2d(global_transform, cursor_position.unwrap());

            if position.is_none() {
                return;
            }

            event_writer.send(DrawPixel {
                position: position.unwrap(),
            });
        }

        if event.button == MouseButton::Middle {
            let (_, _, mut projection, mut transform) = camera_q.single_mut();

            projection.scale = 1.;
            transform.translation.x = 0.;
            transform.translation.y = 0.;
        }
    }
}
