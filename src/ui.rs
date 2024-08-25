use std::fs;

use egui_snarl::{ui::SnarlStyle, Snarl};

use crate::{graph::GraphView, node::NodeView, view::NodeViewer};

const SAVE_FOLDER: &str = "data";

pub struct NodeUI<T> {
    snarl: Snarl<T>,
    style: SnarlStyle,
    saving: bool,
    loading: bool,
    filename: String,
}

impl<T> Default for NodeUI<T> {
    fn default() -> Self {
        Self {
            snarl: Default::default(),
            style: Default::default(),
            saving: false,
            loading: false,
            filename: Default::default(),
        }
    }
}

impl<T: NodeView<T> + GraphView<T> + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>>
    NodeUI<T>
{
    pub fn draw_ui(&mut self, ectx: &egui::Context) {
        egui::CentralPanel::default().show(ectx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Clear").clicked() {
                    self.snarl = Snarl::default()
                }

                if ui.button("Save").clicked() {
                    self.saving = true;
                    self.loading = false;
                };

                if self.saving {
                    ui.label("Filename");
                    ui.text_edit_singleline(&mut self.filename);
                    if ui.button("OK").clicked() {
                        let config = serde_json::to_string_pretty(&self.snarl).unwrap();

                        tracing::info!("{}", config);

                        let _ = fs::write(format!("{}/{}", SAVE_FOLDER, &self.filename), config);

                        self.saving = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.saving = false;
                    }
                }

                if ui.button("Load").clicked() {
                    self.loading = true;
                    self.saving = false;
                };

                if self.loading {
                    ui.label("Filename");
                    ui.text_edit_singleline(&mut self.filename);
                    if ui.button("OK").clicked() {
                        let config = fs::read(format!("{}/{}", SAVE_FOLDER, &self.filename));
                        if let Ok(config) = config {
                            if let Ok(snarl) = serde_json::from_slice(&config) {
                                self.snarl = snarl;
                            }
                        }

                        self.loading = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.loading = false;
                    }
                }
            });

            self.snarl
                .show(&mut NodeViewer, &self.style, egui::Id::new("snarl"), ui);
        });
    }
}
