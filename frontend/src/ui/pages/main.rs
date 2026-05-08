use crate::context::Context;
use crate::errors::FrontendError;
use crate::ui::errors::InputError;
use backend::Input;
use egui::{Grid, RichText, TextEdit};

#[derive(Debug, Default)]
pub struct MainPage {
    a: String,
    b: String,

    journal_output: String,
}

impl MainPage {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.horizontal(|ui| {
            self.inputs(ui);
            self.navigation(ui, context);
        });
        ui.add_space(10.0);
        self.runner(ui, context);
        ui.add_space(10.0);
        self.journal(ui, context);
    }

    fn navigation(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button(egui_phosphor::regular::INFO).clicked() {
                context.ui_state.switch_to_info();
            }
            if ui.button(egui_phosphor::regular::GEAR).clicked() {
                context.ui_state.switch_to_settings();
            }
        });
    }

    fn inputs(&mut self, ui: &mut egui::Ui) {
        Grid::new("MAIN_INPUTS").num_columns(2).show(ui, |ui| {
            ui.label(RichText::new("A:").size(18.0));
            ui.text_edit_singleline(&mut self.a);
            ui.end_row();

            ui.label(RichText::new("B:").size(18.0));
            ui.text_edit_singleline(&mut self.b);
            ui.end_row();
        });
    }

    fn runner(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            // Button that runs solver
            if ui
                .button(
                    RichText::new(format!("{} SOLVE", egui_phosphor::regular::GEAR))
                        .size(18.0),
                )
                .clicked()
            {
                let input = match self.validate_inputs() {
                    Ok(value) => value,
                    Err(error) => {
                        let _ = context.errors_tx.try_send(FrontendError::Input(error));
                        return;
                    },
                };

                // TODO
            }
        });
    }

    fn journal(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.add_sized(
            ui.available_size(),
            TextEdit::multiline(&mut self.journal_output).interactive(false),
        );
    }

    fn validate_inputs(&self) -> Result<Input, InputError> {
        if self.a.is_empty() || self.b.is_empty() {
            return Err(InputError::Empty);
        }

        let a = self.a.parse::<isize>().map_err(InputError::Parse)?;
        let b = self.b.parse::<isize>().map_err(InputError::Parse)?;

        if a >= b {
            return Err(InputError::Order);
        }

        Ok(Input { a, b })
    }
}
