use bevy::prelude::*;

#[derive(Component)]
pub struct Layer {
    pub image_handle: Handle<Image>,
}

impl Layer {
    pub fn new(image_handle: Handle<Image>) -> Layer {
        Layer { image_handle }
    }
}
