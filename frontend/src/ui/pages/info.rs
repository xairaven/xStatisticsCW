use crate::context::Context;
use egui::Color32;

#[derive(Debug)]
pub struct InfoPage {
    version: semver::Version,
}

impl Default for InfoPage {
    fn default() -> Self {
        Self {
            version: semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(
                semver::Version {
                    major: 0,
                    minor: 0,
                    patch: 1,
                    pre: Default::default(),
                    build: Default::default(),
                },
            ),
        }
    }
}

impl InfoPage {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button(egui_phosphor::regular::KEY_RETURN).clicked() {
                context.ui_state.switch_to_main();
            }
        });

        ui.add_space(50.0);
        ui.vertical_centered_justified(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(format!("xStatisticsCW v{}", self.version))
                    .size(25.0)
                    .color(Color32::GREEN),
            ));
            ui.label("Solver of a specific task in Probability Theory. Control Work.");

            ui.add_space(20.0);

            ui.label("Developer: Alex Kovalov");

            ui.add_space(20.0);

            ui.hyperlink_to(
                "Check out the code on GitHub!",
                "https://github.com/xairaven/xStatisticsCW",
            );
            ui.hyperlink_to(
                "*Latest release*",
                "https://github.com/xairaven/xStatisticsCW/releases",
            );
        });
    }
}
