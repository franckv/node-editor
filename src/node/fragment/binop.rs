use egui_snarl::ui::PinInfo;

use crate::node::fragment::Node;
use crate::node::{NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

impl Default for Ops {
    fn default() -> Self {
        Ops::Add
    }
}

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct BinOpNode {
    pub op: Ops,
    pub a: f32,
    pub b: f32,
}

impl BinOpNode {
    pub fn value(&self) -> NodeValue {
        let value = match self.op {
            Ops::Add => self.a + self.b,
            Ops::Sub => self.a - self.b,
            Ops::Mul => self.a * self.b,
            Ops::Div => {
                if self.b == 0. {
                    0.
                } else {
                    self.a / self.b
                }
            }
        };

        NodeValue::F32(value)
    }

    pub fn set_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(value) => {
                if index == 0 {
                    self.a = value;
                } else {
                    self.b = value;
                }
            }
            NodeValue::Vec2(value) => {
                if index == 0 {
                    self.a = value.x;
                } else {
                    self.b = value.y;
                }
            }
            NodeValue::Vec3(value) => {
                if index == 0 {
                    self.a = value.x;
                } else {
                    self.b = value.y;
                }
            }
        }
    }
}

const INPUTS: [NodeValueType; 2] = [NodeValueType::F32, NodeValueType::F32];
const OUTPUTS: [NodeValueType; 1] = [NodeValueType::F32];

impl NodeView<Node> for BinOpNode {
    fn title(&self) -> String {
        "Binary Op".to_string()
    }

    fn inputs(&self) -> &[NodeValueType] {
        &INPUTS
    }

    fn outputs(&self) -> &[NodeValueType] {
        &OUTPUTS
    }

    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<Node>) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.op))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.op, Ops::Add, "Add");
                ui.selectable_value(&mut self.op, Ops::Sub, "Sub");
                ui.selectable_value(&mut self.op, Ops::Mul, "Mul");
                ui.selectable_value(&mut self.op, Ops::Div, "Div");
            });
    }

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            let (r_index, remote_node) = &remotes[0];
            let new_value = match remote_node {
                Node::Float(value) => value.value(),
                Node::BinOp(value) => value.value(),
                Node::CameraPosition(value) => value.value(*r_index),
            };
            self.set_value(index, new_value);

            ui.label(new_value.to_string());
            Node::get_pin_float_connected()
        }
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        ui.label(self.value().to_string());
        if remotes.len() > 0 {
            Node::get_pin_float_connected()
        } else {
            Node::get_pin_float_disconnected()
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!();
    }
}
