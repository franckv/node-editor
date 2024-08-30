mod nodes;

use std::{fmt::Debug, fs, path::Path};

use egui_snarl::{ui::SnarlStyle, Snarl};

use node_compiler::{GraphCompiler, NodeCompile, ScriptGenerator};
use node_model::NodeData;

use crate::{
    graph::GraphView,
    view::{NodeView, NodeViewer},
};

#[derive(PartialEq)]
enum FileFormat {
    Json,
    Ron,
}

pub struct NodeUI<T, G, S> {
    snarl: Snarl<T>,
    style: SnarlStyle,
    compiler: GraphCompiler<G, S>,
    show_script: bool,
    format: FileFormat,
}

impl<T, G, S> Default for NodeUI<T, G, S> {
    fn default() -> Self {
        Self {
            snarl: Default::default(),
            style: Default::default(),
            compiler: Default::default(),
            show_script: false,
            format: FileFormat::Ron,
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
        G: ScriptGenerator<S>,
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

                let mut script = G::script(&code);

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
                    self.save_graph(path);
                }
            };

            if ui.button("Load").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.load_graph(path);
                }
            };

            if ui.button("Show script").clicked() {
                self.show_script = true;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.selectable_value(&mut self.format, FileFormat::Ron, "Ron");
                ui.selectable_value(&mut self.format, FileFormat::Json, "Json");
                ui.label("Format: ");
            });
        });
    }

    pub fn load_graph<P: AsRef<Path> + Debug>(&mut self, path: P) {
        let config = fs::read(path);
        if let Ok(config) = config {
            let snarl = match self.format {
                FileFormat::Json => serde_json::from_slice(&config).map_err(|err| err.to_string()),
                FileFormat::Ron => ron::de::from_bytes(&config).map_err(|err| err.code.to_string()),
            };
            match snarl {
                Ok(snarl) => self.snarl = snarl,
                Err(err) => tracing::warn!("Error loading graph: {}", err),
            }
        }
    }

    pub fn save_graph<P: AsRef<Path> + Debug>(&self, path: P) {
        let config = match self.format {
            FileFormat::Json => serde_json::to_string_pretty(&self.snarl).unwrap(),
            FileFormat::Ron => {
                ron::ser::to_string_pretty(&self.snarl, ron::ser::PrettyConfig::default()).unwrap()
            }
        };

        let _ = fs::write(path, config);
    }
}
