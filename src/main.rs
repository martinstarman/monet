pub mod monet;
use monet::Monet;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("Monet", options, Box::new(|_| Ok(Box::<Monet>::default())))
}
