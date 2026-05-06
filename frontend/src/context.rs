use crate::config::Config;
use crate::errors::ProjectError;
use crate::logs::LogLevel;
use crate::ui::pages::Page;
use crossbeam::channel::{Receiver, Sender};
use egui::Theme;

#[derive(Debug)]
pub struct Context {
    pub ui_state: UiState,
    pub settings: RuntimeSettings,
    pub config: Config,

    // Channels
    pub errors_tx: Sender<ProjectError>,
    pub errors_rx: Receiver<ProjectError>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        let (errors_tx, errors_rx) = crossbeam::channel::unbounded();

        Self {
            ui_state: UiState::default(),
            settings: RuntimeSettings::from(&config),
            config,

            errors_tx,
            errors_rx,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeSettings {
    pub app_id: String,
    pub log_level: LogLevel,
    pub theme: Theme,
}

impl From<&Config> for RuntimeSettings {
    fn from(config: &Config) -> Self {
        Self {
            app_id: config.app_id.clone(),
            log_level: config.log_level,
            theme: config.theme,
        }
    }
}

#[derive(Debug, Default)]
pub struct UiState {
    pub page: Page,
}

impl UiState {
    pub fn switch_to_main(&mut self) {
        self.page = Page::Main;
    }

    pub fn switch_to_settings(&mut self) {
        self.page = Page::Settings;
    }

    pub fn switch_to_info(&mut self) {
        self.page = Page::Info;
    }
}
