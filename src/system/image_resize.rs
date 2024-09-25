use bevy::prelude::{
  Assets, Commands, DetectChanges, Entity, EventWriter, Image, Query, Res, ResMut,
};

use crate::{
  component::layer::Layer, event::add_layer::AddLayer, resource::image_dimensions::ImageDimensions,
};

pub fn image_resize(
  mut commands: Commands,
  layers: Query<(Entity, &Layer)>,
  mut images: ResMut<Assets<Image>>,
  image_dimensions: Res<ImageDimensions>,
  mut event_writer: EventWriter<AddLayer>,
) {
  if !image_dimensions.is_changed() {
    return;
  }

  for (entity, layer) in &layers {
    let image = images.get(&layer.image_handle);

    if image.is_none() {
      continue;
    }

    // TODO: do not remove&create but update in place and preserve content
    images.remove(&layer.image_handle);
    commands.entity(entity).despawn();
    event_writer.send(AddLayer {});
  }
}
