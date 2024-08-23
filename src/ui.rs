use egui_snarl::{ui::SnarlStyle, Snarl};

use crate::{node::NodeView, view::NodeViewer};

pub struct NodeUI<T> {
    snarl: Snarl<T>,
    style: SnarlStyle,
}

impl<T> Default for NodeUI<T> {
    fn default() -> Self {
        Self {
            snarl: Default::default(),
            style: Default::default(),
        }
    }
}

impl<T: NodeView<T> + Clone + serde::Serialize> NodeUI<T> {
    pub fn draw_ui(&mut self, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Clear").clicked() {
                    self.snarl = Snarl::default()
                }

                if ui.button("Save").clicked() {
                    let config = serde_json::to_string(&self.snarl).unwrap();

                    tracing::info!("{}", config);
                }
            });
            self.snarl
                .show(&mut NodeViewer, &self.style, egui::Id::new("snarl"), ui);
        });
    }
}
