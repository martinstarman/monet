use bevy::{
  input::{mouse::MouseWheel, ButtonInput},
  prelude::{Camera2d, EventReader, KeyCode, OrthographicProjection, Query, Res, With},
};
use bevy_egui::EguiContexts;

pub fn camera_zoom(
  mut events: EventReader<MouseWheel>,
  mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
  keys: Res<ButtonInput<KeyCode>>,
  mut egui_contexts: EguiContexts,
) {
  if egui_contexts.ctx_mut().is_pointer_over_area()
    || keys.pressed(KeyCode::ShiftLeft)
    || keys.pressed(KeyCode::ControlLeft)
  {
    return;
  }

  for event in events.read() {
    let mut projection = camera.single_mut();

    if event.y < 0. {
      projection.scale *= 1.25;
    } else {
      projection.scale /= 1.15;
    }
  }
}
