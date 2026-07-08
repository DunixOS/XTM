use crate::app::App;
use crate::ui::widgets::log;

#[derive(Debug, Default, Clone)]
pub struct MessagesPage {
    message_recipient: String,
    message_body: String,
}

impl MessagesPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
        ui.vertical(|ui| {
            ui.heading("Messages");
            ui.label("Compose a message payload for the selected token and submit it through the backend workflow.");
            ui.horizontal(|ui| {
                ui.label("Recipient");
                ui.text_edit_singleline(&mut self.message_recipient);
            });
            ui.label("Body");
            ui.add(egui::TextEdit::multiline(&mut self.message_body).desired_rows(8));
            egui::ComboBox::from_label("Token")
                .selected_text(&app.selected_token)
                .show_ui(ui, |ui| {
                    for token in &app.tokens {
                        ui.selectable_value(&mut app.selected_token, token.gamertag.clone(), token.gamertag.clone());
                    }
                });
            if ui.button("Send").clicked() {
                let recipient = self.message_recipient.trim().to_string();
                let body = self.message_body.trim().to_string();
                if recipient.is_empty() || body.is_empty() {
                    app.status_message = "Recipient and body are required before sending.".to_string();
                    app.log_entries.push("Recipient and body are required before sending.".to_string());
                } else {
                    app.progress = 0.0;
                    app.status_message = format!("Submitting message to {}", recipient);
                    app.log_entries.push(format!("Submitting message to {}", recipient));
                    app.start_message_submission(recipient, body);
                }
            }
            ui.separator();
            log::panel(ui, app);
        });
    }
}
