use crate::app::App;
use crate::ui::widgets::log;

#[derive(Debug, Default, Clone)]
pub struct FollowPage {
    follow_target: String,
}

impl FollowPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
        ui.vertical(|ui| {
            ui.heading("Follow automation");
            ui.label("Prepare a local follow workflow for the selected account.");
            ui.horizontal(|ui| {
                ui.label("Target");
                ui.text_edit_singleline(&mut self.follow_target);
            });
            egui::ComboBox::from_label("Token")
                .selected_text(&app.selected_token)
                .show_ui(ui, |ui| {
                    for token in &app.tokens {
                        ui.selectable_value(&mut app.selected_token, token.gamertag.clone(), token.gamertag.clone());
                    }
                });
            if ui.button("Start").clicked() {
                app.progress = 0.0;
                app.status_message = format!("Starting follow run for {}", self.follow_target);
                app.log_entries.push(format!("Follow run started for {}", self.follow_target));
                app.start_follow_workflow(self.follow_target.clone(), None);
            }

            let progress_value = (app.progress.clamp(0.0, 100.0) / 100.0).clamp(0.0, 1.0);
            ui.add(egui::ProgressBar::new(progress_value).text(format!("{:.0}%", app.progress.clamp(0.0, 100.0))));
            ui.separator();
            log::panel(ui, app);
        });
    }
}
