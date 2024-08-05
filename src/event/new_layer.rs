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

use crate::{component::layer::Layer, resource::image_dimension::ImageDimensions};

use super::set_active_layer::SetActiveLayer;

#[derive(Event)]
pub struct NewLayer {}

pub fn new_layer(
    mut event_reader: EventReader<NewLayer>,
    mut event_writer: EventWriter<SetActiveLayer>,
    mut layers_q: Query<&mut Layer>,
    mut commands: Commands,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimensions>,
) {
    for _ in event_reader.read() {
        for mut layer in &mut layers_q {
            layer.active = false;
        }

        let layer_index = layers_q.iter().len();
        let mut layer_name = String::from("layer #");
        layer_name.push_str(layer_index.to_string().as_str());

        let width = image_dimension_r.width;
        let height = image_dimension_r.height;

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
        let image_handle = images_r.add(image);

        let layer = Layer::new(
            layer_name,
            layer_index as u32,
            image_handle.clone(),
            true,
            true,
        );

        commands.spawn((
            layer,
            SpriteBundle {
                texture: image_handle.clone(),
                sprite: Sprite {
                    flip_y: true,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., layer_index as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        event_writer.send(SetActiveLayer {
            layer_index: layer_index as u32,
        });
    }
}
