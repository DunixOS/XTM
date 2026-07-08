use crate::app::App;
use egui::{Frame, Margin, Slider};

#[derive(Debug, Default, Clone)]
pub struct SettingsPage {
    show_advanced: bool,
    notifications_enabled: bool,
    auto_refresh: bool,
    compact_mode: bool,
    notes: String,
}

impl SettingsPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
        ui.vertical(|ui| {
            ui.heading("Settings");
            ui.label("Adjust the desktop experience and appearance without touching backend services.");
            Frame::default().inner_margin(Margin::same(12)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Theme");
                    egui::ComboBox::from_label("Palette")
                        .selected_text(app.theme.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.theme, crate::theme::ThemeMode::Dark, "Dark");
                            ui.selectable_value(&mut app.theme, crate::theme::ThemeMode::Light, "Light");
                        });
                    ui.checkbox(&mut self.show_advanced, "Show advanced options");
                    ui.checkbox(&mut self.notifications_enabled, "Enable notifications");
                    ui.checkbox(&mut self.auto_refresh, "Auto refresh workspace");
                    ui.checkbox(&mut self.compact_mode, "Compact layout");
                });
            });
            Frame::default().inner_margin(Margin::same(12)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Appearance");
                    ui.horizontal(|ui| {
                        ui.label("UI scale");
                        ui.add(Slider::new(&mut app.ui_scale, 0.8..=1.4).text("x"));
                    });
                    if ui.button("Apply settings").clicked() {
                        app.settings.notifications_enabled = self.notifications_enabled;
                        app.settings.auto_refresh = self.auto_refresh;
                        app.settings.compact_mode = self.compact_mode;
                        app.status_message = "Settings applied locally.".to_string();
                        app.log_entries.push("Settings updated.".to_string());
                    }
                });
            });
            if self.show_advanced {
                Frame::default().inner_margin(Margin::same(12)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label("General preferences");
                        ui.label("Advanced controls are applied to the persisted runtime state.");
                        ui.text_edit_multiline(&mut self.notes);
                    });
                });
            }
        });
    }
}
