use std::marker::PhantomData;

use egui_snarl::ui::PinInfo;

use crate::node::{Connector, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Vec2Node<T> {
    x: f32,
    y: f32,
    node_type: PhantomData<T>,
}

impl<T> Default for Vec2Node<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            node_type: Default::default(),
        }
    }
}

impl<T> Vec2Node<T> {
    pub fn value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

const INPUTS: [Connector; 0] = [];
const OUTPUTS: [Connector; 3] = [
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
        ty: NodeValueType::Vec2,
        label: "xy",
        editable: false,
    },
];

impl<T: NodeView<T>> NodeView<T> for Vec2Node<T> {
    fn out_value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else {
            NodeValue::Vec2((self.x, self.y).into())
        }
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "Vec2".to_string()
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

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) {
        unimplemented!();
    }

    fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, T)>) -> PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo {
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
        unimplemented!();
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> PinInfo {
        unimplemented!()
    }
}
