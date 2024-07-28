mod component;
mod event;
mod gui;
mod resource;
mod system;

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
use gui::{bottom_bar::bottom_bar, left_sidebar::left_sidebar, top_menu_bar::top_menu_bar};
use resource::{image_dimension::ImageDimension, pixel_color::PixelColor};
use system::image_resize::image_resize;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.35, 0.35, 0.38)))
        .init_resource::<PixelColor>()
        .init_resource::<ImageDimension>()
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
        .add_systems(Update, (left_sidebar, bottom_bar, top_menu_bar))
        .add_systems(Update, (mouse_wheel, mouse_click, draw_pixel))
        .add_systems(Update, image_resize)
        .add_event::<DrawPixel>()
        .run()
}

// TODO: move somewhere
fn setup_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn setup_image(
    mut commands: Commands,
    mut images_r: ResMut<Assets<Image>>,
    image_dimension_r: Res<ImageDimension>,
) {
    let width = image_dimension_r.width;
    let height = image_dimension_r.height;

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
    let image_handle = images_r.add(image);

    let entity_id = commands
        .spawn(SpriteBundle {
            texture: image_handle.clone(),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            ..default()
        })
        .id();

    let layer = Layer::new(image_handle, entity_id);

    commands.spawn(layer);
}
