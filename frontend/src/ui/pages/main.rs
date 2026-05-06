use crate::context::Context;

#[derive(Debug)]
pub struct MainPage;

impl MainPage {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button(egui_phosphor::regular::INFO).clicked() {
                context.ui_state.switch_to_info();
            }
            if ui.button(egui_phosphor::regular::GEAR).clicked() {
                context.ui_state.switch_to_settings();
            }
        });
    }
}
