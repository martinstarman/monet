use bevy::prelude::{Component, Entity, Handle, Image};

#[derive(Component)]
pub struct Layer {
    /// layer name
    pub name: String,

    /// layer index
    pub index: u32,

    /// layer to draw on
    pub active: bool,

    /// image resource reference
    pub image_handle: Handle<Image>,

    /// spawned sprite id
    pub entity_id: Entity,
}

impl Layer {
    pub fn new(name: String, index: u32, image_handle: Handle<Image>, entity_id: Entity) -> Layer {
        Layer {
            name,
            index,
            active: false,
            image_handle,
            entity_id,
        }
    }
}
