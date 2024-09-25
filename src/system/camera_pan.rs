use bevy::{
  input::mouse::MouseWheel,
  prelude::{ButtonInput, Camera2d, EventReader, KeyCode, Query, Res, Transform, With},
};
use bevy_egui::EguiContexts;

pub fn camera_pan(
  mut events: EventReader<MouseWheel>,
  mut camera: Query<&mut Transform, With<Camera2d>>,
  keys: Res<ButtonInput<KeyCode>>,
  mut egui_contexts: EguiContexts,
) {
  if egui_contexts.ctx_mut().is_pointer_over_area() {
    return;
  }

  for event in events.read() {
    let mut transform = camera.single_mut();

    if keys.pressed(KeyCode::ShiftLeft) {
      transform.translation.y += if event.y < 0. { -1. } else { 1. };
    }

    if keys.pressed(KeyCode::ControlLeft) {
      transform.translation.x += if event.y < 0. { -1. } else { 1. };
    }
  }
}
