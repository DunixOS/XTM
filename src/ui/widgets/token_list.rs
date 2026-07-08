use crate::app::App;
use egui::{Frame, Margin, RichText};

pub fn table(ui: &mut egui::Ui, app: &mut App, search_query: &str) {
    let mut filtered = app.tokens.iter().filter(|token| {
        search_query.is_empty()
            || token.gamertag.to_lowercase().contains(&search_query.to_lowercase())
            || token.xuid.contains(search_query)
    });

    egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
        ui.vertical(|ui| {
            Frame::default().inner_margin(Margin::same(8)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Gamertag").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Expiration").strong());
                        ui.label(RichText::new("Status").strong());
                        ui.label(RichText::new("XUID").strong());
                    });
                });
            });

            for token in filtered.by_ref() {
                Frame::default().inner_margin(Margin::same(8)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(&token.gamertag).clicked() {
                            app.selected_token = token.gamertag.clone();
                            app.status_message = format!("Selected {}", token.gamertag);
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(&token.expiration);
                            ui.label(&token.status);
                            ui.label(&token.xuid);
                        });
                    });
                });
            }
        });
    });
}
