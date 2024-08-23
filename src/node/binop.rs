use egui_snarl::ui::PinInfo;

use super::Node;

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

    pub fn title() -> String {
        "Binary Op".to_string()
    }

    pub fn inputs() -> usize {
        2
    }

    pub fn outputs() -> usize {
        1
    }

    pub fn connect(other: &Node) -> bool {
        match other {
            Node::Output(_) => true,
            Node::BinOp(_) => true,
            _ => false,
        }
    }

    pub fn has_body() -> bool {
        true
    }

    pub fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<Node>) {
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.op))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.op, Ops::Add, "Add");
                ui.selectable_value(&mut self.op, Ops::Sub, "Sub");
                ui.selectable_value(&mut self.op, Ops::Mul, "Mul");
                ui.selectable_value(&mut self.op, Ops::Div, "Div");
            });
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<Node>) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            let remote_node = &remotes[0];
            let new_value = match remote_node {
                Node::Float(value) => value.value(),
                Node::BinOp(value) => value.value(),
                _ => unimplemented!(),
            };
            if index == 0 {
                self.a = new_value;
            } else {
                self.b = new_value;
            }
            ui.label(Node::format_float(new_value));
            Node::get_pin_float_connected()
        }
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<Node>) -> PinInfo {
        ui.label(Node::format_float(self.value()));
        if remotes.len() > 0 {
            Node::get_pin_float_connected()
        } else {
            Node::get_pin_float_disconnected()
        }
    }
}
