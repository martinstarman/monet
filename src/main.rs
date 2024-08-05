mod component;
mod event;
mod gui;
mod resource;
mod system;

use bevy::{
  prelude::{
    App, AppExit, ClearColor, DefaultPlugins, ImagePlugin, IntoSystemConfigs, PluginGroup, Startup,
    Update, Window, WindowPlugin,
  },
  window::WindowResolution,
};
use bevy_egui::EguiPlugin;

use event::{
  add_layer::{add_layer, AddLayer},
  paint_at::{paint_at, PaintAt},
  set_active_layer::{set_active_layer, SetActiveLayer},
};
use gui::{
  panel_bottom::panel_bottom, panel_top::panel_top, sidebar_left::sidebar_left,
  sidebar_right::sidebar_right,
};
use resource::{color::Color, image_dimensions::ImageDimensions};
use system::{
  camera_pan::camera_pan, camera_reset::camera_reset, camera_setup::camera_setup,
  camera_zoom::camera_zoom, image_paint::image_paint, image_resize::image_resize,
  image_setup::image_setup,
};

fn main() -> AppExit {
  App::new()
    .insert_resource(ClearColor(bevy::prelude::Color::srgb(0.35, 0.35, 0.38)))
    .init_resource::<Color>()
    .init_resource::<ImageDimensions>()
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
    .add_systems(Startup, (camera_setup, image_setup))
    .add_systems(
      Update,
      (
        // gui
        panel_top,
        panel_bottom,
        sidebar_left,
        sidebar_right,
        // systems
        camera_pan,
        camera_reset,
        camera_zoom,
        image_paint,
        image_resize,
        // events
        add_layer,
        paint_at,
        set_active_layer,
      )
        .chain(),
    )
    .add_event::<AddLayer>()
    .add_event::<PaintAt>()
    .add_event::<SetActiveLayer>()
    .run()
}
