use crate::config::{Config, Theme};
use crate::errors::FrontendError;
use crate::logs::LogLevel;
use crate::ui::pages::Page;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct Context {
    pub ui_state: UiState,
    pub settings: RuntimeSettings,
    pub config: Config,

    // Channels
    pub errors_tx: Sender<FrontendError>,
    pub errors_rx: Receiver<FrontendError>,
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

    pub fn save_settings(&mut self) {
        self.config = Config::from(&self.settings);

        if let Err(error) = self.config.save_to_file() {
            let _ = self.errors_tx.try_send(error);
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

impl From<&RuntimeSettings> for Config {
    fn from(settings: &RuntimeSettings) -> Self {
        Self {
            app_id: settings.app_id.clone(),
            log_level: settings.log_level,
            theme: settings.theme,
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
