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
            let position = event.position;
            let width = image.width() as f32;
            let index = (position.y * width) + position.x;

            // TODO: position is from (-w/2,-h/2)×(w/2,h/2)
            // TODO: out of bounds check
            image.data[index as usize] = 255;
        }
    }
}
