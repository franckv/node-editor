use egui_snarl::ui::PinInfo;

use crate::node::{math::MathNode, NodeView};

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
}

impl NodeView<MathNode> for FloatNode {
    fn title(&self) -> String {
        "Float".to_string()
    }

    fn inputs(&self) -> usize {
        0
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
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<MathNode>) {
        unimplemented!();
    }

    fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<MathNode>) -> PinInfo {
        unimplemented!();
    }

    fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<MathNode>) -> PinInfo {
        ui.add(egui::DragValue::new(self.value_mut()));
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
