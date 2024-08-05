use bevy::{
  input::{mouse::MouseWheel, ButtonInput},
  prelude::{Camera2d, EventReader, KeyCode, OrthographicProjection, Query, Res, With},
};
use bevy_egui::EguiContexts;

pub fn camera_zoom(
  mut mouse_wheel_event_reader: EventReader<MouseWheel>,
  mut camera_q: Query<&mut OrthographicProjection, With<Camera2d>>,
  keys_r: Res<ButtonInput<KeyCode>>,
  mut egui_contexts: EguiContexts,
) {
  if egui_contexts.ctx_mut().is_pointer_over_area()
    || keys_r.pressed(KeyCode::ShiftLeft)
    || keys_r.pressed(KeyCode::ControlLeft)
  {
    return;
  }

  for event in mouse_wheel_event_reader.read() {
    let mut projection = camera_q.single_mut();

    if event.y < 0. {
      projection.scale *= 1.25;
    } else {
      projection.scale /= 1.15;
    }
  }
}
