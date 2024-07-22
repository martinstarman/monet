use bevy::{input::mouse::MouseButtonInput, prelude::*};

use super::draw_pixel::DrawPixel;

pub fn mouse_click(
    mut event_reader: EventReader<MouseButtonInput>,
    mut event_writer: EventWriter<DrawPixel>,
    windows_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    for event in event_reader.read() {
        if event.button != MouseButton::Left {
            return;
        }

        let window = windows_q.single();
        let cursor_position = window.cursor_position();

        if cursor_position.is_none() {
            return;
        }

        let (camera, camera_transform) = camera_q.single();
        let position = camera.viewport_to_world_2d(camera_transform, cursor_position.unwrap());

        if position.is_none() {
            return;
        }

        event_writer.send(DrawPixel {
            position: position.unwrap(),
        });
    }
}
