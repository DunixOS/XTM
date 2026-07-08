use crate::app::App;

#[derive(Debug, Default, Clone)]
pub struct AboutPage;

impl AboutPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, _app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                egui::Frame::default().inner_margin(30.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("XTM");
                        ui.label("Desktop token workspace");
                    });
                });
                ui.vertical(|ui| {
                    ui.heading("About");
                    ui.label("Version 0.1.0");
                    ui.label("License GPL-3.0-only");
                    ui.label("Copyright 2026");
                    ui.label("Built around the configured token source and API-driven actions");
                    ui.label("Credits: desktop shell, storage layer, and Xbox workflow integration");
                });
            });
        });
    }
}
