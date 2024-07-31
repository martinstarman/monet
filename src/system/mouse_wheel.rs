use bevy::{
    input::mouse::MouseWheel,
    prelude::{
        ButtonInput, Camera2d, EventReader, KeyCode, OrthographicProjection, Query, Res, Transform,
        With,
    },
};
use bevy_egui::EguiContexts;

pub fn mouse_wheel(
    mut event_reader: EventReader<MouseWheel>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    keys_r: Res<ButtonInput<KeyCode>>,
    mut egui_contexts: EguiContexts,
) {
    if egui_contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    for event in event_reader.read() {
        let (mut projection, mut transform) = camera_q.single_mut();

        // vertical panning
        if keys_r.pressed(KeyCode::ShiftLeft) {
            if event.y < 0. {
                transform.translation.y -= 1.;
            } else {
                transform.translation.y += 1.;
            }

            return;
        }

        // horizontal panning
        if keys_r.pressed(KeyCode::ControlLeft) {
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
