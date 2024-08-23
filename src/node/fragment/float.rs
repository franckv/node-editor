use egui_snarl::ui::PinInfo;

use crate::node::{fragment::Node, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct FloatNode {
    value: f32,
}

impl FloatNode {
    pub fn value(&self) -> NodeValue {
        NodeValue::F32(self.value)
    }

    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

const INPUTS: [NodeValueType; 0] = [];
const OUTPUTS: [NodeValueType; 1] = [NodeValueType::F32];

impl NodeView<Node> for FloatNode {
    fn title(&self) -> String {
        "Float".to_string()
    }

    fn inputs(&self) -> &[NodeValueType] {
        &INPUTS
    }

    fn outputs(&self) -> &[NodeValueType] {
        &OUTPUTS
    }

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<Node>) {
        unimplemented!();
    }

    fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, Node)>) -> PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        ui.add(egui::DragValue::new(self.value_mut()));
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
