use bevy::prelude::EventWriter;

use crate::event::add_layer::AddLayer;

pub fn image_setup(mut event_writer: EventWriter<AddLayer>) {
  event_writer.send(AddLayer {});
}
