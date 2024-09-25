use bevy::prelude::{Changed, Entity, Event, EventReader, Query};

use crate::component::layer::Layer;

#[derive(Event)]
pub struct ChangeActiveLayer {
  pub id: Entity,
}

pub fn change_active_layer(
  mut events: EventReader<ChangeActiveLayer>,
  mut layers: Query<(Entity, &mut Layer), Changed<Layer>>,
) {
  for event in events.read() {
    for (id, mut layer) in &mut layers {
      layer.is_active = id == event.id;
    }
  }
}
