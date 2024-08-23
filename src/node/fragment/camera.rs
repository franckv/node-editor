use crate::node::{fragment::FragmentNode, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct CameraPositionNode {
    x: f32,
    y: f32,
    z: f32,
}

impl CameraPositionNode {
    pub fn value(&self, index: usize) -> f32 {
        if index == 0 {
            self.x
        } else if index == 1 {
            self.y
        } else if index == 2 {
            self.z
        } else {
            unimplemented!()
        }
    }

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

    pub fn value_label(&self, index: usize) -> String {
        if index == 0 {
            "x".to_string()
        } else if index == 1 {
            "y".to_string()
        } else if index == 2 {
            "z".to_string()
        } else {
            unimplemented!()
        }
    }
}

impl NodeView<FragmentNode> for CameraPositionNode {
    fn title(&self) -> String {
        "CameraPosition".to_string()
    }

    fn inputs(&self) -> usize {
        0
    }

    fn outputs(&self) -> usize {
        3
    }

    fn connect(&self, other: &FragmentNode) -> bool {
        match other {
            FragmentNode::BinOp(_) => true,
            _ => false,
        }
    }

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _: &mut egui::Ui, _: &Vec<FragmentNode>) {
        todo!()
    }

    fn show_input(
        &mut self,
        _: &mut egui::Ui,
        _: usize,
        _: &Vec<(usize, FragmentNode)>,
    ) -> egui_snarl::ui::PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, FragmentNode)>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label(self.value_label(index));
        ui.add(egui::DragValue::new(self.value_mut(index)));
        if remotes.len() > 0 {
            FragmentNode::get_pin_float_connected()
        } else {
            FragmentNode::get_pin_float_disconnected()
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<FragmentNode> {
        unimplemented!()
    }
}
