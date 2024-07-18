mod event;
mod gui;
mod layer;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        view::RenderLayers,
    },
};
use bevy_egui::EguiPlugin;

use event::{mouse_click::mouse_click, mouse_wheel::mouse_wheel};
use gui::top_menu_bar::top_menu_bar;
use layer::Layer;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.38)))
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_camera, setup_image))
        .add_systems(Update, top_menu_bar)
        .add_systems(Update, (mouse_wheel, mouse_click))
        .run();
}
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PIXEL_PERFECT_LAYERS));
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
        ..default()
    });

    let layer = Layer::new(handle.clone());

    commands.spawn(layer);
}
