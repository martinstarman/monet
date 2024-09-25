use bevy::{
  math::Vec3,
  prelude::{
    Assets, Commands, Event, EventReader, EventWriter, Image, Query, Res, ResMut, Transform,
  },
  render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::TextureFormatPixelInfo,
  },
  sprite::{Sprite, SpriteBundle},
};

use crate::{component::layer::Layer, resource::image_dimensions::ImageDimensions};

use super::change_active_layer::ChangeActiveLayer;

#[derive(Event)]
pub struct AddLayer {}

pub fn add_layer(
  mut events: EventReader<AddLayer>,
  mut event_writer: EventWriter<ChangeActiveLayer>,
  layers: Query<&Layer>,
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  image_dimensions: Res<ImageDimensions>,
) {
  for _ in events.read() {
    let width = image_dimensions.width;
    let height = image_dimensions.height;

    let size = Extent3d {
      width,
      height,
      ..Default::default()
    };
    let dimension = TextureDimension::D2;
    let data: Vec<u8> =
      vec![0; width as usize * height as usize * TextureFormat::Rgba8Unorm.pixel_size()];
    let format = TextureFormat::Rgba8Unorm;
    let asset_usage = RenderAssetUsages::default();
    let image = Image::new(size, dimension, data, format, asset_usage);
    let image_handle = images.add(image);

    let position = layers.iter().len() as u32;

    let layer = Layer {
      name: String::from("new layer"),
      image_handle: image_handle.clone(),
      is_active: true,
      is_visible: true,
    };

    let id = commands
      .spawn((
        layer,
        SpriteBundle {
          texture: image_handle.clone(),
          sprite: Sprite {
            flip_y: true,
            ..Default::default()
          },
          transform: Transform {
            translation: Vec3::new(0., 0., position as f32),
            ..Default::default()
          },
          ..Default::default()
        },
      ))
      .id();

    // set new layer as active
    event_writer.send(ChangeActiveLayer { id });
  }
}
