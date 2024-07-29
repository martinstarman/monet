use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Color {
    pub value: [f32; 4],
}

impl Default for Color {
    fn default() -> Self {
        Color {
            value: [1., 0., 0., 1.],
        }
    }
}
