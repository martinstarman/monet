use bevy::prelude::*;

use crate::component::layer::Layer;

#[derive(Event)]
pub struct DrawPixel {
    pub position: Vec2,
}

pub fn draw_pixel(
    mut event_reader: EventReader<DrawPixel>,
    layer_q: Query<&Layer>,
    mut images_r: ResMut<Assets<Image>>,
) {
    for event in event_reader.read() {
        let layer = layer_q.single();

        if let Some(image) = images_r.get_mut(&layer.image_handle) {
            let width = image.width() as i32;
            let height = image.height() as i32;

            // move position from [-w/2,-h/2]x[w/2,h/2] to [0,0]x[w,h]
            let x = (event.position.x.floor() as i32) + (width / 2);
            let y = (event.position.y.floor() as i32) + (height / 2);

            let index = ((y * width) + x) as usize;

            if index < image.data.len() {
                image.data[index] = 255; // TODO: color
            }
        }
    }
}
