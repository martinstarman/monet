use bevy::prelude::{Component, Handle, Image};

#[derive(Component)]
pub struct Layer {
  /// layer name
  pub name: String,

  /// image resource reference
  pub image_handle: Handle<Image>,

  /// layer to draw on
  pub is_active: bool,

  /// layer visibility
  pub is_visible: bool,
}
