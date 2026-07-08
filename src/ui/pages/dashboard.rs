use crate::app::App;
use egui::{Frame, Margin, RichText};

#[derive(Debug, Default, Clone)]
pub struct DashboardPage {
    activity: Vec<ActivityItem>,
}

#[derive(Debug, Clone)]
struct ActivityItem {
    label: String,
    time: String,
}

impl DashboardPage {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);

        Frame::default().inner_margin(Margin::same(16)).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new("Workspace overview").size(24.0));
                ui.label("Monitor the token inventory and the current backend-driven workflow state.");
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    let token_count = app.tokens.len().to_string();
                    stat_card(ui, "Tokens", &format!("{token_count} loaded"), "From the configured token source");
                    stat_card(ui, "Follow", "Waiting", "Start the workflow with a target account");
                    stat_card(ui, "Messages", "Pending", "Use the selected token for live submission");
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("Recent activity");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Open log").clicked() {
                            app.status_message = "Activity log opened from the current workflow state.".to_string();
                        }
                    });
                });

                if self.activity.is_empty() {
                    self.activity.push(ActivityItem {
                        label: format!("Loaded {} token(s) from the token source", app.tokens.len()),
                        time: "Now".to_string(),
                    });
                }

                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        for item in &self.activity {
                            ui.horizontal(|ui| {
                                ui.label(&item.label);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(&item.time);
                                });
                            });
                        }
                    });
                });

                ui.separator();
                ui.heading("Quick actions");
                ui.horizontal(|ui| {
                    if ui.button("Review tokens").clicked() {
                        app.current_page = crate::app::Page::Tokens;
                        app.status_message = "Tokens page selected for inspection.".to_string();
                    }
                    if ui.button("Prepare follow").clicked() {
                        app.current_page = crate::app::Page::Follow;
                        app.status_message = "Follow workflow page selected.".to_string();
                    }
                    if ui.button("Draft message").clicked() {
                        app.current_page = crate::app::Page::Messages;
                        app.status_message = "Message workflow page selected.".to_string();
                    }
                });
            });
        });
    }
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: &str, hint: &str) {
    Frame::default().inner_margin(Margin::same(14)).show(ui, |ui| {
        ui.set_min_width(150.0);
        ui.vertical(|ui| {
            ui.label(RichText::new(title).strong());
            ui.heading(value);
            ui.label(hint);
        });
    });
}
