use crate::app::App;
use egui::{Frame, Margin};

pub fn panel(ui: &mut egui::Ui, app: &App) {
    Frame::default().inner_margin(Margin::same(12)).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label("Activity log");
            egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    for entry in &app.log_entries {
                        ui.label(entry);
                    }
                });
            });
        });
    });
}
