mod nodes;

use std::{fmt::Debug, fs};

use egui_snarl::{ui::SnarlStyle, Snarl};

use node_model::NodeData;
use node_compiler::{GraphCompiler, NodeCompile};

use crate::{
    graph::GraphView,
    view::{NodeView, NodeViewer},
};

pub struct NodeUI<T, G, S> {
    snarl: Snarl<T>,
    style: SnarlStyle,
    compiler: GraphCompiler<G, S>,
    show_script: bool,
}

impl<T, G, S> Default for NodeUI<T, G, S> {
    fn default() -> Self {
        Self {
            snarl: Default::default(),
            style: Default::default(),
            compiler: Default::default(),
            show_script: false,
        }
    }
}

impl<
        T: NodeData<T>
            + NodeView<T>
            + GraphView<T>
            + NodeCompile<T, G, S>
            + Clone
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>,
        G,
        S: Debug,
    > NodeUI<T, G, S>
{
    pub fn draw_ui(&mut self, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            self.menu_bar(ui);

            self.show_script(ectx);

            self.snarl
                .show(&mut NodeViewer, &self.style, egui::Id::new("snarl"), ui);
        });
    }

    pub fn show_script(&mut self, ectx: &egui::Context) {
        egui::Window::new("Script")
            .open(&mut self.show_script)
            .show(ectx, |ui| {
                let code = self.compiler.compile(&self.snarl);

                let mut script = String::from("");
                for fragment in code {
                    script += &format!("[{:?}] {}\n", &fragment.section, &fragment.code);
                }
                ui.text_edit_multiline(&mut script);
            });
    }

    pub fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            if ui.button("Clear").clicked() {
                self.snarl = Snarl::default()
            }

            if ui.button("Save").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    let config = serde_json::to_string_pretty(&self.snarl).unwrap();

                    tracing::info!("{}", config);

                    let _ = fs::write(path, config);
                }
            };

            if ui.button("Load").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let config = fs::read(path);
                    if let Ok(config) = config {
                        if let Ok(snarl) = serde_json::from_slice(&config) {
                            self.snarl = snarl;
                        }
                    }
                }
            };

            if ui.button("Show script").clicked() {
                self.show_script = true;
            }
        });
    }
}
