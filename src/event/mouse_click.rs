use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::layer::Layer;

pub fn mouse_click(
    mut event_reader: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    layer_q: Query<&Layer>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in event_reader.read() {
        if event.button == MouseButton::Left {
            let (camera, camera_transform) = camera_q.single();

            if let Some(cursor_position) = windows.single().cursor_position() {
                if let Some(_world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    let layer = layer_q.single();

                    if let Some(image) = images.get_mut(&layer.handle) {
                        image.data[55] = 255;
                    }
                }
            }
        }
    }
}
