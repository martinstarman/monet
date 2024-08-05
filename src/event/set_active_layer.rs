use bevy::prelude::{Changed, Entity, Event, EventReader, Query};

use crate::component::layer::Layer;

#[derive(Event)]
pub struct SetActiveLayer {
  pub layer_entity: Entity,
}

pub fn set_active_layer(
  mut set_active_layer_event_reader: EventReader<SetActiveLayer>,
  mut layers_q: Query<(Entity, &mut Layer), Changed<Layer>>,
) {
  for event in set_active_layer_event_reader.read() {
    for (entity, mut layer) in &mut layers_q {
      layer.active = entity == event.layer_entity;
    }
  }
}
