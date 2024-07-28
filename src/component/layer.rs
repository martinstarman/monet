use bevy::prelude::*;

#[derive(Component)]
pub struct Layer {
    /// image resource reference
    pub image_handle: Handle<Image>,

    /// spawned sprite id
    pub entity_id: Entity,
}

impl Layer {
    pub fn new(image_handle: Handle<Image>, entity_id: Entity) -> Layer {
        Layer {
            image_handle,
            entity_id,
        }
    }
}
