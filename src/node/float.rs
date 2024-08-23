use egui_snarl::ui::PinInfo;

use super::Node;

#[derive(Clone, Default, Debug)]
pub struct FloatNode {
    value: f32,
}

impl FloatNode {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    pub fn title() -> String {
        "Float".to_string()
    }

    pub fn inputs() -> usize {
        0
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

    pub fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<Node>) -> PinInfo {
        unimplemented!();
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<Node>) -> PinInfo {
        ui.add(egui::DragValue::new(self.value_mut()));
        if remotes.len() > 0 {
            Node::get_pin_float_connected()
        } else {
            Node::get_pin_float_disconnected()
        }
    }
}
