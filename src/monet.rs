pub struct Monet {}

impl Default for Monet {
    fn default() -> Self {
        Monet {}
    }
}

impl Monet {
    pub fn ui_left_panel(&mut self, _ui: &mut egui::Ui) {}

    pub fn ui_canvas(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (mut response, _painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::drag(),
            );

            let to_screen = egui::emath::RectTransform::from_to(
                egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
                response.rect,
            );
            let from_screen = to_screen.inverse();

            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let _canvas_pos = from_screen * pointer_pos;

                // TODO

                response.mark_changed();
            }

            response
        });
    }
}

impl eframe::App for Monet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .max_width(32.)
            .resizable(false)
            .show(ctx, |ui| {
                self.ui_left_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_canvas(ui);
        });
    }
}
