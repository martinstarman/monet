use bevy::prelude::*;

#[derive(Resource)]
pub struct ImageDimension {
    pub width: u32,
    pub height: u32,
}

impl Default for ImageDimension {
    fn default() -> Self {
        ImageDimension {
            width: 100,
            height: 100,
        }
    }
}
