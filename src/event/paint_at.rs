use bevy::{
  prelude::{Assets, Event, EventReader, Image, Query, Res, ResMut},
  render::texture::TextureFormatPixelInfo,
};

use crate::{component::layer::Layer, resource::color::Color};

#[derive(Event)]
pub struct PaintAt {
  pub x: f32,
  pub y: f32,
}

pub fn paint_at(
  mut events: EventReader<PaintAt>,
  layers: Query<&Layer>,
  mut images: ResMut<Assets<Image>>,
  color: Res<Color>,
) {
  for event in events.read() {
    for layer in &layers {
      if !layer.is_active || !layer.is_visible {
        continue;
      }

      if let Some(image) = images.get_mut(&layer.image_handle) {
        let x = event.x.floor() as i32;
        let y = event.y.floor() as i32;
        let width = image.width() as i32;
        let height = image.height() as i32;

        if x < -width / 2 || x >= width / 2 {
          return;
        }

        if y < -height / 2 || y >= height / 2 {
          return;
        }

        // move position from [-w/2,-h/2]x[w/2,h/2] to [0,0]x[w,h]
        let x = x + width / 2;
        let y = y + height / 2;

        let index = ((y * width) + x) as usize * image.texture_descriptor.format.pixel_size();

        image.data[index] = (color.r * 255.0) as u8;
        image.data[index + 1] = (color.g * 255.0) as u8;
        image.data[index + 2] = (color.b * 255.0) as u8;
        image.data[index + 3] = (color.a * 255.0) as u8;
      }
    }
  }
}
