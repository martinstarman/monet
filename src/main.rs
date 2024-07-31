mod component;
mod event;
mod gui;
mod resource;
mod system;

use bevy::{
    prelude::{
        App, AppExit, ClearColor, DefaultPlugins, ImagePlugin, PluginGroup, Startup, Update,
        Window, WindowPlugin,
    },
    window::WindowResolution,
};
use bevy_egui::EguiPlugin;

use event::paint::{paint, Paint};
use gui::{
    bottom_bar::bottom_bar, left_sidebar::left_sidebar, right_sidebar::right_sidebar,
    top_menu_bar::top_menu_bar,
};
use resource::{color::Color, image_dimension::ImageDimension};
use system::{
    image_resize::image_resize, left_mouse_button_down::left_mouse_button_down,
    middle_mouse_button_click::middle_mouse_button_click, mouse_wheel::mouse_wheel,
    setup_camera::setup_camera, setup_image::setup_image,
};

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(bevy::prelude::Color::srgb(0.35, 0.35, 0.38)))
        .init_resource::<Color>()
        .init_resource::<ImageDimension>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Monet".into(),
                        resolution: WindowResolution::new(800., 600.),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_camera, setup_image))
        .add_systems(
            Update,
            (left_sidebar, right_sidebar, bottom_bar, top_menu_bar),
        )
        .add_systems(Update, paint)
        .add_systems(
            Update,
            (
                image_resize,
                middle_mouse_button_click,
                left_mouse_button_down,
                mouse_wheel,
            ),
        )
        .add_event::<Paint>()
        .run()
}
