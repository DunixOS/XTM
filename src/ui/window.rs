use crate::app::{App, Page};
use egui::{Frame, Margin, RichText};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("xtm_topbar")
        .min_height(72.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(RichText::new(app.current_page.label()).size(24.0));
                    ui.label("API-driven token workspace");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Refresh").clicked() {
                        app.progress = 100.0;
                        app.status_message = "Workspace refreshed.".to_string();
                        app.log_entries.push("Workspace refreshed locally.".to_string());
                    }
                    ui.label(format!("Theme: {}", app.theme));
                });
            });
        });

    egui::SidePanel::left("xtm_sidebar")
        .resizable(false)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.set_min_height(640.0);
            Frame::default().inner_margin(Margin::same(16)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading("XTM");
                    ui.label("Token workspace");
                    ui.separator();
                    for page in [
                        Page::Dashboard,
                        Page::Tokens,
                        Page::Follow,
                        Page::Messages,
                        Page::Reports,
                        Page::Settings,
                        Page::About,
                    ] {
                        let selected = app.current_page == page;
                        if ui.selectable_label(selected, page.label()).clicked() {
                            app.current_page = page;
                            app.status_message = format!("Viewing {}", page.label());
                        }
                    }
                    ui.separator();
                    ui.label("Backend-ready interface");
                    ui.label("Connected to token source");
                });
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.set_min_width(600.0);
        Frame::default().inner_margin(Margin::same(20)).show(ui, |ui| {
            ui.vertical(|ui| {
                match app.current_page {
                    Page::Dashboard => {
                        let mut page = std::mem::take(&mut app.dashboard_page);
                        page.ui(ui, app);
                        app.dashboard_page = page;
                    }
                    Page::Tokens => {
                        let mut page = std::mem::take(&mut app.tokens_page);
                        page.ui(ui, app);
                        app.tokens_page = page;
                    }
                    Page::Follow => {
                        let mut page = std::mem::take(&mut app.follow_page);
                        page.ui(ui, app);
                        app.follow_page = page;
                    }
                    Page::Messages => {
                        let mut page = std::mem::take(&mut app.messages_page);
                        page.ui(ui, app);
                        app.messages_page = page;
                    }
                    Page::Reports => {
                        let mut page = std::mem::take(&mut app.reports_page);
                        page.ui(ui, app);
                        app.reports_page = page;
                    }
                    Page::Settings => {
                        let mut page = std::mem::take(&mut app.settings_page);
                        page.ui(ui, app);
                        app.settings_page = page;
                    }
                    Page::About => {
                        let mut page = std::mem::take(&mut app.about_page);
                        page.ui(ui, app);
                        app.about_page = page;
                    }
                }
            });
        });
    });

    egui::TopBottomPanel::bottom("xtm_status")
        .min_height(40.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&app.status_message).italics());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Progress {}%", app.progress.round() as i32));
                    ui.label("API ready");
                });
            });
        });
}
