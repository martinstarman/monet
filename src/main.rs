mod component;
mod event;
mod gui;
mod resource;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::TextureFormatPixelInfo,
    },
};
use bevy_egui::EguiPlugin;

use component::layer::Layer;
use event::{
    draw_pixel::{draw_pixel, DrawPixel},
    mouse_click::mouse_click,
    mouse_wheel::mouse_wheel,
};
use gui::{left_sidebar::left_sidebar, top_menu_bar::top_menu_bar};
use resource::pixel_color::PixelColor;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.38)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Monet".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_camera, setup_image))
        .add_systems(Update, (left_sidebar, top_menu_bar))
        .add_systems(Update, (mouse_wheel, mouse_click, draw_pixel))
        .add_event::<DrawPixel>()
        .init_resource::<PixelColor>()
        .run()
}

// TODO: move somewhere
fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn setup_image(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let width = 10;
    let height = 10;

    let size = Extent3d {
        width,
        height,
        ..Default::default()
    };
    let dimension = TextureDimension::D2;
    let data: Vec<u8> =
        vec![255; width as usize * height as usize * TextureFormat::Rgba8Unorm.pixel_size()];
    let format = TextureFormat::Rgba8Unorm;
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
