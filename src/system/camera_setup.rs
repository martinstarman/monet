use bevy::prelude::{Camera2dBundle, Commands};

pub fn camera_setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
