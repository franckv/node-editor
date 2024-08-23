use egui_snarl::ui::PinInfo;

use super::Node;

#[derive(Clone, Default, Debug)]
pub struct SubNode {
    pub a: f32,
    pub b: f32,
}

impl SubNode {
    pub fn value(&self) -> f32 {
        self.a - self.b
    }

    pub fn title() -> String {
        "Sub".to_string()
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
            Node::Float(_) => false,
            Node::OpAdd(_) => true,
            Node::OpSub(_) => true,
        }
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<Node>) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            let remote_node = &remotes[0];
            let new_value = match remote_node {
                Node::Float(value) => value.value(),
                Node::OpAdd(value) => value.value(),
                Node::OpSub(value) => value.value(),
                _ => unimplemented!(),
            };
            if index == 0 {
                self.a = new_value;
            } else {
                self.b = new_value;
            }
            ui.label(format!("{}", new_value));
            Node::get_pin_float_connected()
        }
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<Node>) -> PinInfo {
        ui.label(format!("{}", self.value()));
        if remotes.len() > 0 {
            Node::get_pin_float_connected()
        } else {
            Node::get_pin_float_disconnected()
        }
    }
}
