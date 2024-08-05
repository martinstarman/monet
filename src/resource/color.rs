use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Default for Color {
  fn default() -> Self {
    Color {
      r: 1.,
      g: 1.,
      b: 1.,
      a: 1.,
    }
  }
}
