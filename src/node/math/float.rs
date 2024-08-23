use egui_snarl::ui::PinInfo;

use crate::node::math::MathNode;
use crate::node::NodeView;

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

    pub fn connect(other: &MathNode) -> bool {
        match other {
            MathNode::Output(_) => true,
            MathNode::BinOp(_) => true,
            _ => false,
        }
    }

    pub fn has_body() -> bool {
        false
    }

    pub fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<MathNode>) {
        unimplemented!();
    }

    pub fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<MathNode>) -> PinInfo {
        unimplemented!();
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<MathNode>) -> PinInfo {
        ui.add(egui::DragValue::new(self.value_mut()));
        if remotes.len() > 0 {
            MathNode::get_pin_float_connected()
        } else {
            MathNode::get_pin_float_disconnected()
        }
    }
}
