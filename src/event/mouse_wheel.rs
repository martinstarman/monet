use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn mouse_wheel(
    mut event_reader: EventReader<MouseWheel>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for event in event_reader.read() {
        let (mut projection, mut transform) = camera.single_mut();

        // vertical panning
        if keys.pressed(KeyCode::ShiftLeft) {
            if event.y < 0. {
                transform.translation.y -= 1.;
            } else {
                transform.translation.y += 1.;
            }

            return;
        }

        // horizontal panning
        if keys.pressed(KeyCode::ControlLeft) {
            if event.y < 0. {
                transform.translation.x -= 1.;
            } else {
                transform.translation.x += 1.;
            }

            return;
        }

        // zoom
        if event.y < 0. {
            projection.scale *= 1.25;
        } else {
            projection.scale /= 1.15;
        }
    }
}
