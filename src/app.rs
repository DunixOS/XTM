use std::sync::Arc;

use reqwest::Client;
use tokio::sync::mpsc;

use crate::api::xbox::error::XboxApiError;
use crate::api::xbox::models::FollowEvent;

use crate::api::xbox::client::XboxApiClient;
use crate::api::xbox::models::{FollowProgress, MessageEvent};
use crate::models::{AppSettings, TokenRecord};
use crate::storage::{settings, tokens};
use crate::theme::ThemeMode;
use crate::ui;
use crate::ui::pages::{AboutPage, DashboardPage, FollowPage, MessagesPage, ReportsPage, SettingsPage, TokensPage};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Dashboard,
    Tokens,
    Follow,
    Messages,
    Reports,
    Settings,
    About,
}

impl Page {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Tokens => "Tokens",
            Self::Follow => "Follow",
            Self::Messages => "Messages",
            Self::Reports => "Reports",
            Self::Settings => "Settings",
            Self::About => "About",
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub current_page: Page,
    pub theme: ThemeMode,
    pub settings: AppSettings,
    pub selected_token: String,
    pub ui_scale: f32,
    pub progress: f32,
    pub status_message: String,
    pub log_entries: Vec<String>,
    pub tokens: Vec<TokenRecord>,
    pub dashboard_page: DashboardPage,
    pub tokens_page: TokensPage,
    pub follow_page: FollowPage,
    pub messages_page: MessagesPage,
    pub reports_page: ReportsPage,
    pub settings_page: SettingsPage,
    pub about_page: AboutPage,
    pub follow_progress: Option<FollowProgress>,
    pub follow_sender: Option<mpsc::UnboundedSender<FollowEvent>>,
    pub follow_receiver: Option<mpsc::UnboundedReceiver<FollowEvent>>,
    pub message_sender: Option<mpsc::UnboundedSender<MessageEvent>>,
    pub message_receiver: Option<mpsc::UnboundedReceiver<MessageEvent>>,
    pub follow_client: Option<Arc<XboxApiClient>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_follow_events();
        self.process_message_events();
        ctx.set_visuals(self.theme.to_visuals());
        ctx.set_pixels_per_point(self.ui_scale);
        ui::window::show(self, ctx);

        if let Err(err) = self.persist() {
            self.status_message = format!("Persistence failed: {err}");
            self.log_entries.push(err.to_string());
        }
    }
}

impl App {
    pub fn initialize_follow_runtime(&mut self) {
        if self.follow_client.is_none() {
            let client = Client::builder().build().ok();
            if let Some(client) = client {
                self.follow_client = Some(Arc::new(XboxApiClient::new(client)));
            }
        }
    }

    pub fn start_follow_workflow(&mut self, target: String, limit: Option<usize>) {
        self.initialize_follow_runtime();
        self.follow_progress = None;
        self.progress = 0.0;
        self.status_message = format!("Starting follow run for {}", target);

        let total_tokens = limit.unwrap_or(self.tokens.len()).min(self.tokens.len());
        self.follow_progress = Some(FollowProgress {
            total_tokens,
            successful_follows: 0,
            failed_follows: 0,
            expired_tokens: 0,
            progress: 0.0,
            current_target: target.clone(),
            log: vec![],
        });

        let (sender, receiver) = mpsc::unbounded_channel::<FollowEvent>();
        self.follow_sender = Some(sender.clone());
        self.follow_receiver = Some(receiver);

        if let Some(client) = self.follow_client.clone() {
            let tokens = self.tokens.clone();
            let follow_target = target.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build follow runtime");

                let _ = runtime.block_on(client.run_follow_workflow(&tokens, &follow_target, limit, Some(sender)));
            });
        }
    }

    pub fn start_message_submission(&mut self, recipient: String, body: String) {
        self.initialize_follow_runtime();
        let (sender, receiver) = mpsc::unbounded_channel::<MessageEvent>();
        self.message_sender = Some(sender.clone());
        self.message_receiver = Some(receiver);

        if let Some(client) = self.follow_client.clone() {
            let tokens = self.tokens.clone();
            let selected_token = self.selected_token.clone();
            let recipient_for_request = recipient.clone();
            let body_for_request = body.clone();
            let recipient_for_event = recipient.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build message runtime");

                let result = runtime.block_on(async move {
                    let token = tokens.iter().find(|token| token.gamertag == selected_token).cloned().or_else(|| tokens.first().cloned());
                    match token {
                        Some(token) => client.send_message(&token, &recipient_for_request, &body_for_request).await,
                        None => Err(XboxApiError::MissingData("no token available".to_string())),
                    }
                });

                let event = match result {
                    Ok(detail) => MessageEvent::Completed { recipient: recipient_for_event.clone(), succeeded: true, detail },
                    Err(err) => MessageEvent::Completed { recipient: recipient_for_event, succeeded: false, detail: err.to_string() },
                };
                let _ = sender.send(event);
            });
        }
    }

    fn process_follow_events(&mut self) {
        if let Some(receiver) = self.follow_receiver.as_mut() {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    FollowEvent::Started { gamertag } => {
                        self.status_message = format!("Starting follow for {gamertag}");
                        self.log_entries.push(format!("Starting follow for {gamertag}"));
                    }
                    FollowEvent::Completed { gamertag, succeeded } => {
                        if let Some(progress) = self.follow_progress.as_mut() {
                            if succeeded {
                                progress.successful_follows += 1;
                            } else {
                                progress.failed_follows += 1;
                            }
                            progress.progress = if progress.total_tokens == 0 {
                                100.0
                            } else {
                                ((progress.successful_follows + progress.failed_follows + progress.expired_tokens) as f32 / progress.total_tokens as f32) * 100.0
                            };
                            self.progress = progress.progress.clamp(0.0, 100.0);
                        }
                        if succeeded {
                            self.log_entries.push(format!("{gamertag}: follow succeeded"));
                        } else {
                            self.log_entries.push(format!("{gamertag}: follow failed"));
                        }
                    }
                    FollowEvent::Expired { gamertag, .. } => {
                        if let Some(progress) = self.follow_progress.as_mut() {
                            progress.expired_tokens += 1;
                            progress.progress = if progress.total_tokens == 0 {
                                100.0
                            } else {
                                ((progress.successful_follows + progress.failed_follows + progress.expired_tokens) as f32 / progress.total_tokens as f32) * 100.0
                            };
                            self.progress = progress.progress.clamp(0.0, 100.0);
                        }
                        self.log_entries.push(format!("{gamertag}: token expired"));
                    }
                    FollowEvent::Finished { progress } => {
                        self.follow_progress = Some(progress.clone());
                        self.progress = progress.progress.clamp(0.0, 100.0);
                        self.status_message = format!("Completed follow run for {}", progress.current_target);
                        self.log_entries.extend(progress.log.iter().cloned());
                        self.log_entries.push("Follow run completed.".to_string());
                    }
                }
            }
        }
    }

    fn process_message_events(&mut self) {
        if let Some(receiver) = self.message_receiver.as_mut() {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    MessageEvent::Completed { recipient, succeeded, detail } => {
                        self.progress = if succeeded { 100.0 } else { 0.0 };
                        if succeeded {
                            self.status_message = format!("Message sent to {recipient}");
                            self.log_entries.push(format!("Message sent to {recipient}"));
                        } else {
                            self.status_message = format!("Message submission failed for {recipient}: {detail}");
                            self.log_entries.push(format!("Message submission failed for {recipient}: {detail}"));
                        }
                    }
                }
            }
        }
    }

    fn persist(&mut self) -> anyhow::Result<()> {
        self.settings.theme = self.theme;
        self.settings.ui_scale = self.ui_scale;
        self.settings.selected_token = Some(self.selected_token.clone());

        settings::save_settings(&self.settings)?;
        tokens::save_tokens(&self.tokens)?;
        Ok(())
    }

    fn load_state() -> (AppSettings, Vec<TokenRecord>, String) {
        let settings = settings::load_settings().unwrap_or_default();
        let tokens = tokens::load_tokens().unwrap_or_default();
        let selected_token = settings.selected_token.clone().unwrap_or_else(|| tokens.first().map(|token| token.gamertag.clone()).unwrap_or_default());
        (settings, tokens, selected_token)
    }
}

impl Default for App {
    fn default() -> Self {
        let (settings, tokens, selected_token) = Self::load_state();

        Self {
            current_page: Page::Dashboard,
            theme: settings.theme,
            settings: settings.clone(),
            selected_token: selected_token.clone(),
            ui_scale: 1.0,
            progress: 42.0,
            status_message: format!("Loaded {} token record(s) from the configured source.", tokens.len()),
            log_entries: vec![
                format!("Loaded {} token record(s) from the configured source.", tokens.len()),
                "Follow, message, and report actions will use the active backend workflow when available.".to_string(),
            ],
            tokens,
            dashboard_page: DashboardPage::default(),
            tokens_page: TokensPage::default(),
            follow_page: FollowPage::default(),
            messages_page: MessagesPage::default(),
            reports_page: ReportsPage::default(),
            settings_page: SettingsPage::default(),
            about_page: AboutPage::default(),
            follow_progress: None,
            follow_sender: None,
            follow_receiver: None,
            message_sender: None,
            message_receiver: None,
            follow_client: None,
        }
    }
}

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native("XTM", options, Box::new(|_cc| Ok(Box::<App>::default())))
}
