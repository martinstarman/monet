use bevy::prelude::*;

#[derive(Component)]
pub struct Layer {
    pub handle: Handle<Image>,
}

impl Layer {
    pub fn new(handle: Handle<Image>) -> Layer {
        Layer { handle }
    }
}
