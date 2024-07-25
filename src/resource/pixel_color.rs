use bevy::prelude::*;

#[derive(Resource)]
pub struct PixelColor {
    pub value: [f32; 4],
}

impl Default for PixelColor {
    fn default() -> Self {
        PixelColor {
            value: [1., 1., 1., 1.],
        }
    }
}
