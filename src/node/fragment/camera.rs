use std::marker::PhantomData;

use crate::node::{NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CameraPositionNode<T> {
    x: f32,
    y: f32,
    z: f32,
    node_type: PhantomData<T>,
}

impl<T> Default for CameraPositionNode<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            node_type: Default::default(),
        }
    }
}

impl<T> CameraPositionNode<T> {
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

impl<T: NodeView<T>> NodeView<T> for CameraPositionNode<T> {
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

    fn show_body(&mut self, _: &mut egui::Ui, _: &Vec<T>) {
        todo!()
    }

    fn show_input(
        &mut self,
        _: &mut egui::Ui,
        _: usize,
        _: &Vec<(usize, T)>,
    ) -> egui_snarl::ui::PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> egui_snarl::ui::PinInfo {
        let (ty, label) = self.outputs()[index];
        ui.label(label);
        if index < 3 {
            ui.add(egui::DragValue::new(self.value_mut(index)));
            if remotes.len() > 0 {
                T::get_node_pin(ty, true)
            } else {
                T::get_node_pin(ty, false)
            }
        } else {
            ui.label(self.out_value(index).to_string());

            if remotes.len() > 0 {
                T::get_node_pin(ty, true)
            } else {
                T::get_node_pin(ty, false)
            }
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<T> {
        unimplemented!()
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> egui_snarl::ui::PinInfo {
        unimplemented!()
    }
}
