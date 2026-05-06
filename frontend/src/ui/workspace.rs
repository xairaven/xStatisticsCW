use crate::context::Context;
use crate::ui::pages::Page;
use crate::ui::pages::info::InfoPage;
use crate::ui::pages::main::MainPage;
use crate::ui::pages::settings::SettingsPage;

#[derive(Debug)]
pub struct Workspace {
    pub main: MainPage,
    pub settings: SettingsPage,
    pub info: InfoPage,
}

impl Workspace {
    pub fn new(_context: &Context) -> Self {
        Self {
            main: MainPage,
            settings: SettingsPage,
            info: Default::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        match &context.ui_state.page {
            Page::Main => self.main.show(ui, context),
            Page::Settings => self.settings.show(ui, context),
            Page::Info => self.info.show(ui, context),
        }
    }
}
