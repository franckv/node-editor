use std::marker::PhantomData;

use crate::node::{Connector, NodeValue, NodeValueType, NodeView};

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

const INPUTS: [Connector; 0] = [];
const OUTPUTS: [Connector; 4] = [
    Connector {
        ty: NodeValueType::F32,
        label: "x",
        editable: true,
    },
    Connector {
        ty: NodeValueType::F32,
        label: "y",
        editable: true,
    },
    Connector {
        ty: NodeValueType::F32,
        label: "z",
        editable: true,
    },
    Connector {
        ty: NodeValueType::Vec3,
        label: "xyz",
        editable: false,
    },
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

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
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
        let Connector {
            ty,
            label,
            editable,
        } = self.outputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        if editable {
            ui.add(egui::DragValue::new(self.value_mut(index)));
        } else {
            ui.label(self.out_value(index).to_string());
        }

        T::get_node_pin(ty, connected)
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<T> {
        unimplemented!()
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> egui_snarl::ui::PinInfo {
        unimplemented!()
    }
}
