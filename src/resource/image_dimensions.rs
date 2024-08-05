use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ImageDimensions {
  pub width: u32,
  pub height: u32,
}

impl Default for ImageDimensions {
  fn default() -> Self {
    ImageDimensions {
      width: 100,
      height: 100,
    }
  }
}
