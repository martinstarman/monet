use bevy::prelude::{Component, Handle, Image};

#[derive(Component)]
pub struct Layer {
    /// layer name
    pub name: String,

    /// layer index
    pub index: u32,

    /// layer to draw on
    pub active: bool,

    /// layer visibility
    pub visible: bool,

    /// image resource reference
    pub image_handle: Handle<Image>,
}

impl Layer {
    pub fn new(
        name: String,
        index: u32,
        image_handle: Handle<Image>,
        active: bool,
        visible: bool,
    ) -> Layer {
        Layer {
            name,
            index,
            active,
            visible,
            image_handle,
        }
    }
}
