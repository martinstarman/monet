use bevy::prelude::EventWriter;

use crate::event::new_layer::NewLayer;

pub fn setup_image(mut event_writer: EventWriter<NewLayer>) {
    event_writer.send(NewLayer {
        name: None,
        index: None,
        active: Some(true),
        visible: Some(true),
    });
}
