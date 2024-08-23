use egui_snarl::ui::PinInfo;

use crate::node::fragment::FragmentNode;
use crate::node::NodeView;

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
    pub fn value(&self) -> f32 {
        match self.op {
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
        }
    }
}

impl NodeView<FragmentNode> for BinOpNode {
    fn title(&self) -> String {
        "Binary Op".to_string()
    }

    fn inputs(&self) -> usize {
        2
    }

    fn outputs(&self) -> usize {
        1
    }

    fn connect(&self, other: &FragmentNode) -> bool {
        match other {
            FragmentNode::BinOp(_) => true,
            _ => false,
        }
    }

    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<FragmentNode>) {
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
        remotes: &Vec<(usize, FragmentNode)>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            FragmentNode::get_pin_float_disconnected()
        } else {
            let (r_index, remote_node) = &remotes[0];
            let new_value = match remote_node {
                FragmentNode::Float(value) => value.value(),
                FragmentNode::BinOp(value) => value.value(),
                FragmentNode::CameraPosition(value) => value.value(*r_index),
            };
            if index == 0 {
                self.a = new_value;
            } else {
                self.b = new_value;
            }
            ui.label(FragmentNode::format_float(new_value));
            FragmentNode::get_pin_float_connected()
        }
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<(usize, FragmentNode)>,
    ) -> PinInfo {
        ui.label(FragmentNode::format_float(self.value()));
        if remotes.len() > 0 {
            FragmentNode::get_pin_float_connected()
        } else {
            FragmentNode::get_pin_float_disconnected()
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<FragmentNode> {
        unimplemented!();
    }
}
