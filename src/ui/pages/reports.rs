use crate::app::App;
use crate::ui::widgets::log;

#[derive(Debug, Default, Clone)]
pub struct ReportsPage {
    report_target: String,
    report_reason: String,
    notes: String,
}

impl ReportsPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
        ui.vertical(|ui| {
            ui.heading("Reports");
            ui.label("Submit a report using the selected token and the configured backend workflow.");
            ui.horizontal(|ui| {
                ui.label("Target");
                ui.text_edit_singleline(&mut self.report_target);
            });
            egui::ComboBox::from_label("Reason")
                .selected_text(&self.report_reason)
                .show_ui(ui, |ui| {
                    for reason in ["Spamming", "Harassment", "Scam", "Other"] {
                        ui.selectable_value(&mut self.report_reason, reason.to_string(), reason);
                    }
                });
            ui.label("Notes");
            ui.add(egui::TextEdit::multiline(&mut self.notes).desired_rows(6));
            egui::ComboBox::from_label("Token")
                .selected_text(&app.selected_token)
                .show_ui(ui, |ui| {
                    for token in &app.tokens {
                        ui.selectable_value(&mut app.selected_token, token.gamertag.clone(), token.gamertag.clone());
                    }
                });
            if ui.button("Submit").clicked() {
                app.progress = 0.0;
                app.status_message = format!("Preparing report submission for {}", self.report_target);
                app.log_entries.push(format!("Report submission requested for {}", self.report_target));
            }
            ui.separator();
            log::panel(ui, app);
        });
    }
}
