use bevy::{prelude::*, render::texture::TextureFormatPixelInfo};

use crate::{component::layer::Layer, resource::pixel_color::PixelColor};

#[derive(Event)]
pub struct DrawPixel {
    pub position: Vec2,
}

pub fn draw_pixel(
    mut event_reader: EventReader<DrawPixel>,
    layer_q: Query<&Layer>,
    mut images_r: ResMut<Assets<Image>>,
    pixel_color: Res<PixelColor>,
) {
    for event in event_reader.read() {
        let layer = layer_q.single();

        if let Some(image) = images_r.get_mut(&layer.image_handle) {
            let x = event.position.x.floor() as i32;
            let y = event.position.y.floor() as i32;
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

            image.data[index] = (pixel_color.value[0] * 255.0) as u8;
            image.data[index + 1] = (pixel_color.value[1] * 255.0) as u8;
            image.data[index + 2] = (pixel_color.value[2] * 255.0) as u8;
            image.data[index + 3] = (pixel_color.value[3] * 255.0) as u8;
        }
    }
}