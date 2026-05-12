use crate::context::Context;
use crate::errors::FrontendError;
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
    pub fn new(context: &Context) -> Self {
        Self {
            main: MainPage::default(),
            settings: SettingsPage::new(context),
            info: InfoPage::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        match &context.ui_state.page {
            Page::Main => self.main.show(ui, context),
            Page::Settings => self.settings.show(ui, context),
            Page::Info => self.info.show(ui, context),
        }

        // If solving is in progress, constantly drawing UI (waiting for journal updates)
        if context.is_solving_in_process {
            ui.request_repaint();

            // If there's some answer from solver - we need to process it
            if let Ok(answer) = context.solver_rx.try_recv() {
                context.is_solving_in_process = false;

                match answer {
                    Ok(_) => {
                        // TODO: RFD, etc
                    },
                    Err(error) => {
                        let error = FrontendError::Backend(error);
                        let _ = context.errors_tx.try_send(error);
                    },
                }
            }
        }
    }
}
