use bevy::{
    prelude::{Assets, Commands, Image, Res, ResMut, Sprite, SpriteBundle},
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::TextureFormatPixelInfo,
    },
};

use crate::{component::layer::Layer, resource::image_dimension::ImageDimension};

pub fn setup_image(
    mut commands: Commands,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimension>,
) {
    let width = image_dimension_r.width;
    let height = image_dimension_r.height;

    let size = Extent3d {
        width,
        height,
        ..Default::default()
    };
    let dimension = TextureDimension::D2;
    let data: Vec<u8> =
        vec![255; width as usize * height as usize * TextureFormat::Rgba8Unorm.pixel_size()];
    let format = TextureFormat::Rgba8Unorm;
    let asset_usage = RenderAssetUsages::default();
    let image = Image::new(size, dimension, data, format, asset_usage);
    let image_handle = images_r.add(image);

    let entity_id = commands
        .spawn(SpriteBundle {
            texture: image_handle.clone(),
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let layer = Layer::new(image_handle, entity_id);

    commands.spawn(layer);
}
