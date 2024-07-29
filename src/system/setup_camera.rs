use bevy::prelude::{Camera2dBundle, Commands};

pub fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
