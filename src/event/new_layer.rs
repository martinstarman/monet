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
pub struct NewLayer {
    pub name: Option<String>,
    pub index: Option<u32>,
    pub active: Option<bool>,
    pub visible: Option<bool>,
}

pub fn new_layer(
    mut event_reader: EventReader<NewLayer>,
    mut event_writer: EventWriter<SetActiveLayer>,
    layers_q: Query<&Layer>,
    mut commands: Commands,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimensions>,
) {
    for event in event_reader.read() {
        let layer_index = if event.index.is_some() {
            event.index.unwrap()
        } else {
            layers_q.iter().len() as u32
        };

        let layer_name = if event.name.is_some() {
            event.name.clone().unwrap()
        } else {
            let mut name = String::from("layer #");
            name.push_str(layer_index.to_string().as_str());
            name
        };

        let active = if event.active.is_some() {
            event.active.unwrap()
        } else {
            true
        };

        let visible = if event.visible.is_some() {
            event.visible.unwrap()
        } else {
            true
        };

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
            active,
            visible,
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
