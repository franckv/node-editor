use egui_snarl::ui::PinInfo;

use crate::node::math::MathNode;
use crate::node::NodeView;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Default, Debug)]
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

impl NodeView<MathNode> for BinOpNode {
    fn title(&self) -> String {
        "Binary Op".to_string()
    }

    fn inputs(&self) -> usize {
        2
    }

    fn outputs(&self) -> usize {
        1
    }

    fn connect(&self, other: &MathNode) -> bool {
        match other {
            MathNode::Output(_) => true,
            MathNode::BinOp(_) => true,
            _ => false,
        }
    }

    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<MathNode>) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.op))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.op, Ops::Add, "Add");
                ui.selectable_value(&mut self.op, Ops::Sub, "Sub");
                ui.selectable_value(&mut self.op, Ops::Mul, "Mul");
                ui.selectable_value(&mut self.op, Ops::Div, "Div");
            });
    }

    fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<MathNode>) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            MathNode::get_pin_float_disconnected()
        } else {
            let remote_node = &remotes[0];
            let new_value = match remote_node {
                MathNode::Float(value) => value.value(),
                MathNode::BinOp(value) => value.value(),
                _ => unimplemented!(),
            };
            if index == 0 {
                self.a = new_value;
            } else {
                self.b = new_value;
            }
            ui.label(MathNode::format_float(new_value));
            MathNode::get_pin_float_connected()
        }
    }

    fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<MathNode>) -> PinInfo {
        ui.label(MathNode::format_float(self.value()));
        if remotes.len() > 0 {
            MathNode::get_pin_float_connected()
        } else {
            MathNode::get_pin_float_disconnected()
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<MathNode> {
        unimplemented!();
    }
}
