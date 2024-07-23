mod component;
mod event;
mod gui;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_egui::EguiPlugin;

use component::layer::Layer;
use event::{
    draw_pixel::{draw_pixel, DrawPixel},
    mouse_click::mouse_click,
    mouse_wheel::mouse_wheel,
};
use gui::top_menu_bar::top_menu_bar;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.38)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_camera, setup_image))
        .add_systems(Update, top_menu_bar)
        .add_systems(Update, (mouse_wheel, mouse_click, draw_pixel))
        .add_event::<DrawPixel>()
        .run()
}

// TODO: move somewhere
fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn setup_image(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 10,
        height: 10,
        ..Default::default()
    };
    let dimension = TextureDimension::D2;
    let data: Vec<u8> = vec![0; 100];
    let format = TextureFormat::R8Unorm;
    let asset_usage = RenderAssetUsages::default();
    let image = Image::new(size, dimension, data, format, asset_usage);
    let handle = images.add(image);

    commands.spawn(SpriteBundle {
        texture: handle.clone(),
        sprite: Sprite {
            flip_y: true,
            ..default()
        },
        ..default()
    });

    let layer = Layer::new(handle.clone());

    commands.spawn(layer);
}
