use crate::context::Context;

#[derive(Debug)]
pub struct SettingsPage;

impl SettingsPage {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button(egui_phosphor::regular::KEY_RETURN).clicked() {
                context.ui_state.switch_to_main();
            }
        });
    }
}
