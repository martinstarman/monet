use bevy::prelude::{Changed, Event, EventReader, Query};

use crate::component::layer::Layer;

#[derive(Event)]
pub struct SetActiveLayer {
    pub index: u32,
}

pub fn set_active_layer(
    mut event_reader: EventReader<SetActiveLayer>,
    mut layers_q: Query<&mut Layer, Changed<Layer>>,
) {
    for event in event_reader.read() {
        for mut layer in &mut layers_q {
            layer.active = layer.index == event.index;
        }
    }
}
