use bevy::prelude::{Component, Handle, Image};

#[derive(Component)]
pub struct Layer {
  /// layer name
  pub name: String,

  /// layer index
  pub index: u32,

  /// image resource reference
  pub image_handle: Handle<Image>,

  /// layer to draw on
  pub active: bool,

  /// layer visibility
  pub visible: bool,
}
