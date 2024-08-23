use crate::node::{fragment::Node, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct CameraPositionNode {
    x: f32,
    y: f32,
    z: f32,
}

impl CameraPositionNode {
    pub fn value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else if index == 2 {
            &mut self.z
        } else {
            unimplemented!()
        }
    }
}

const INPUTS: [(NodeValueType, &str); 0] = [];
const OUTPUTS: [(NodeValueType, &str); 4] = [
    (NodeValueType::F32, "x"),
    (NodeValueType::F32, "y"),
    (NodeValueType::F32, "z"),
    (NodeValueType::Vec3, "xyz"),
];

impl NodeView<Node> for CameraPositionNode {
    fn out_value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else if index == 2 {
            NodeValue::F32(self.z)
        } else {
            NodeValue::Vec3((self.x, self.y, self.z).into())
        }
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "CameraPosition".to_string()
    }

    fn inputs(&self) -> &[(NodeValueType, &str)] {
        &INPUTS
    }

    fn outputs(&self) -> &[(NodeValueType, &str)] {
        &OUTPUTS
    }

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _: &mut egui::Ui, _: &Vec<Node>) {
        todo!()
    }

    fn show_input(
        &mut self,
        _: &mut egui::Ui,
        _: usize,
        _: &Vec<(usize, Node)>,
    ) -> egui_snarl::ui::PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label(self.outputs()[index].1);
        if index < 3 {
            ui.add(egui::DragValue::new(self.value_mut(index)));
            if remotes.len() > 0 {
                Node::get_pin_float_connected()
            } else {
                Node::get_pin_float_disconnected()
            }
        } else {
            ui.label(self.out_value(index).to_string());

            if remotes.len() > 0 {
                Node::get_pin_vec_connected()
            } else {
                Node::get_pin_vec_disconnected()
            }
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!()
    }
}
