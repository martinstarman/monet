use bevy::prelude::{
    Assets, Commands, DetectChanges, Entity, EventWriter, Image, Query, Res, ResMut,
};

use crate::{
    component::layer::Layer, event::new_layer::NewLayer, resource::image_dimension::ImageDimensions,
};

pub fn image_resize(
    mut commands: Commands,
    layers_q: Query<(Entity, &Layer)>,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimensions>,
    mut event_writer: EventWriter<NewLayer>,
) {
    if !image_dimension_r.is_changed() {
        return;
    }

    for (entity, layer) in &layers_q {
        let image = images_r.get(&layer.image_handle);

        if image.is_none() {
            continue;
        }

        images_r.remove(&layer.image_handle);
        commands.entity(entity).despawn();

        // TODO: preserve data content
        event_writer.send(NewLayer {
            name: Some(layer.name.clone()),
            index: Some(layer.index),
            active: Some(layer.active),
            visible: Some(layer.visible),
        });
    }
}
