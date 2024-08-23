use egui_snarl::{ui::SnarlStyle, Snarl};

use crate::{node::Node, view::NodeViewer};

#[derive(Default)]
pub struct NodeUI {
    snarl: Snarl<Node>,
    style: SnarlStyle,
}

impl NodeUI {
    pub fn draw_ui(&mut self, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            self.snarl
                .show(&mut NodeViewer, &self.style, egui::Id::new("snarl"), ui);
        });
    }
}
