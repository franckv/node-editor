use egui_snarl::ui::PinInfo;

use crate::node::{math::Node, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct Vec2Node {
    x: f32,
    y: f32,
}

impl Vec2Node {
    pub fn values(&self) -> NodeValue {
        NodeValue::Vec2((self.x, self.y).into())
    }

    pub fn value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else {
            NodeValue::Vec2((self.x, self.y).into())
        }
    }

    pub fn value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

const INPUTS: [NodeValueType; 0] = [];
const OUTPUTS: [NodeValueType; 3] = [NodeValueType::F32, NodeValueType::F32, NodeValueType::Vec2];

impl NodeView<Node> for Vec2Node {
    fn title(&self) -> String {
        "Vec2".to_string()
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
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        ui.label(self.values().value_label(index));
        if index < 2 {
            ui.add(egui::DragValue::new(self.value_mut(index)));

            if remotes.len() > 0 {
                Node::get_pin_float_connected()
            } else {
                Node::get_pin_float_disconnected()
            }
        } else {
            ui.label(self.values().to_string());

            if remotes.len() > 0 {
                Node::get_pin_vec_connected()
            } else {
                Node::get_pin_vec_disconnected()
            }
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!();
    }
}
