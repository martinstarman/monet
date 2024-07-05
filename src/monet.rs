pub struct Monet {}

impl Default for Monet {
    fn default() -> Self {
        Monet {}
    }
}

impl eframe::App for Monet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
        });
    }
}
