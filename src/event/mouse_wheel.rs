use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn mouse_wheel(
    mut event_reader: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    for event in event_reader.read() {
        let mut projection = camera.single_mut();

        // TODO: zoom with ctrl
        if event.y < 0. {
            projection.scale *= 1.25;
        } else {
            projection.scale /= 1.15;
        }

        // TODO: vertical scroll on wheel
        // TODO: horizontal scroll on wheel + Shift
    }
}
