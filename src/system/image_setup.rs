use bevy::prelude::EventWriter;

use crate::event::add_layer::AddLayer;

pub fn image_setup(mut add_layer_event_writer: EventWriter<AddLayer>) {
  add_layer_event_writer.send(AddLayer {
    name: None,
    index: None,
    active: Some(true),
    visible: Some(true),
  });
}
