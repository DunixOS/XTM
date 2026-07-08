use crate::app::App;
use crate::ui::widgets::{log, token_list};

#[derive(Debug, Default, Clone)]
pub struct TokensPage {
    search_query: String,
}

impl TokensPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Search");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Refresh").clicked() {
                    if let Ok(tokens) = crate::storage::tokens::load_tokens() {
                        app.tokens = tokens;
                        app.status_message = "Token source refreshed from the configured file.".to_string();
                        app.log_entries.push("Loaded tokens from the configured token source.".to_string());
                    } else {
                        app.status_message = "Failed to refresh the configured token source.".to_string();
                        app.log_entries.push("Unable to load tokens from the configured token source.".to_string());
                    }
                }
                if ui.button("Remove").clicked() {
                    app.tokens.retain(|token| token.gamertag != app.selected_token);
                    app.status_message = "Selected token removed from the active list.".to_string();
                    app.log_entries.push("Removed token from the active list.".to_string());
                }
            });
            ui.separator();
            token_list::table(ui, app, &self.search_query);
            ui.separator();
            log::panel(ui, app);
        });
    }
}
