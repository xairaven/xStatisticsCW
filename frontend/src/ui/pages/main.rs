use crate::context::Context;
use crate::errors::FrontendError;
use crate::ui::errors::InputError;
use backend::{BackendError, Input, Solver};
use egui::{Grid, RichText, ScrollArea, TextEdit};

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
        self.solve_button(ui, context);
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

    fn solve_button(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            // Button that runs solver
            if ui
                .button(
                    RichText::new(format!("{} SOLVE", egui_phosphor::regular::GEAR))
                        .size(18.0),
                )
                .clicked()
            {
                self.run_solver(context);
            }
        });
    }

    fn journal(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        // Check for logs
        if let Ok(message) = context.journaling_rx.try_recv() {
            let message = format!("{}\n", message);
            self.journal_output.push_str(&message);
        }

        ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.journal_output).interactive(false),
                );
            });
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

    fn run_solver(&self, context: &mut Context) {
        let input = match self.validate_inputs() {
            Ok(value) => value,
            Err(error) => {
                let _ = context.errors_tx.try_send(FrontendError::Input(error));
                return;
            },
        };

        context.is_solving_in_process = true;

        // Clone data needed for the thread
        let app_id = context.settings.app_id.trim().to_string();
        let solver_tx = context.solver_tx.clone();
        let journaling_tx = context.journaling_tx.clone();

        // Spawn a background native thread so egui doesn't freeze
        std::thread::spawn(move || {
            // Create a dedicated tokio runtime for the async tasks
            let runtime = match tokio::runtime::Runtime::new() {
                Ok(value) => value,
                Err(error) => {
                    let error = BackendError::Tokio(error);
                    let _ = solver_tx.try_send(Err(error));
                    return;
                },
            };

            // TODO: Max Concurrent tasks
            const MAX_CONCURRENT_TASKS: usize = 3;
            runtime.block_on(async {
                let solver =
                    Solver::new(app_id, journaling_tx.clone(), MAX_CONCURRENT_TASKS);

                let result = solver.run(input).await;

                // Send result back to UI
                match result {
                    Ok(code) => {
                        // TODO: Fix this, now it is like this for debug
                        let _ = journaling_tx.try_send("Solver result: OK".to_string());
                        let _ = solver_tx.send(Ok(code));
                    },
                    Err(error) => {
                        let _ = solver_tx.send(Err(error));
                    },
                }
            });
        });
    }
}
