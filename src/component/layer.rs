use bevy::prelude::{Component, Entity, Handle, Image};

#[derive(Component)]
pub struct Layer {
    /// layer name
    pub name: String,

    /// image resource reference
    pub image_handle: Handle<Image>,

    /// spawned sprite id
    pub entity_id: Entity,
}

impl Layer {
    pub fn new(name: String, image_handle: Handle<Image>, entity_id: Entity) -> Layer {
        Layer {
            image_handle,
            entity_id,
            name,
        }
    }
}
