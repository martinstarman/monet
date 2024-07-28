use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::TextureFormatPixelInfo,
    },
};

use crate::{component::layer::Layer, resource::image_dimension::ImageDimension};

pub fn image_resize(
    mut commands: Commands,
    mut layers_q: Query<&mut Layer>,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimension>,
) {
    if !image_dimension_r.is_changed() {
        return;
    }

    let width = image_dimension_r.width;
    let height = image_dimension_r.height;

    for mut layer in &mut layers_q {
        let image = images_r.get(&layer.image_handle);

        if image.is_none() {
            continue;
        }

        images_r.remove(&layer.image_handle);
        commands.entity(layer.entity_id).despawn();

        // TODO: duplicate code
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

        // TODO: preserve data content

        let entity_id = commands
            .spawn(SpriteBundle {
                texture: image_handle.clone(),
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                ..default()
            })
            .id();

        layer.image_handle = image_handle;
        layer.entity_id = entity_id;
    }
}
