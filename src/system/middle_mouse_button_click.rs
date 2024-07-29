use bevy::{
    input::mouse::MouseButtonInput,
    prelude::{Camera2d, EventReader, MouseButton, OrthographicProjection, Query, Transform, With},
};
use bevy_egui::EguiContexts;

pub fn middle_mouse_button_click(
    mut event_reader: EventReader<MouseButtonInput>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    mut egui_contexts: EguiContexts,
) {
    for event in event_reader.read() {
        if egui_contexts.ctx_mut().is_pointer_over_area() {
            return;
        }

        if event.button == MouseButton::Middle {
            let (mut projection, mut transform) = camera_q.single_mut();

            projection.scale = 1.;
            transform.translation.x = 0.;
            transform.translation.y = 0.;
        }
    }
}
