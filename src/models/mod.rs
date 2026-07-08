mod token;

pub use token::TokenRecord;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub theme: crate::theme::ThemeMode,
    pub ui_scale: f32,
    pub notifications_enabled: bool,
    pub auto_refresh: bool,
    pub compact_mode: bool,
    pub selected_token: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: crate::theme::ThemeMode::Dark,
            ui_scale: 1.0,
            notifications_enabled: false,
            auto_refresh: false,
            compact_mode: false,
            selected_token: None,
        }
    }
}
