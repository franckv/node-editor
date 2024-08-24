use std::marker::PhantomData;

use egui_snarl::ui::PinInfo;

use crate::node::{Connector, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComposeNode<T> {
    x: f32,
    y: f32,
    node_type: PhantomData<T>,
}

impl<T> Default for ComposeNode<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            node_type: Default::default(),
        }
    }
}

impl<T> ComposeNode<T> {
    fn values(&self) -> NodeValue {
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

const INPUTS: [Connector; 2] = [
    Connector {
        ty: NodeValueType::F32,
        label: "x",
        editable: false,
    },
    Connector {
        ty: NodeValueType::F32,
        label: "y",
        editable: false,
    },
];
const OUTPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::Vec2,
    label: "xy",
    editable: false,
}];

impl<T: NodeView<T>> NodeView<T> for ComposeNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::Vec2((self.x, self.y).into())
    }

    fn in_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::None => unimplemented!(),
            NodeValue::F32(value) => {
                if index == 0 {
                    self.x = value;
                } else {
                    self.y = value;
                }
            }
            NodeValue::Vec2(value) => {
                if index == 0 {
                    self.x = value.x;
                } else {
                    self.y = value.y;
                }
            }
            NodeValue::Vec3(value) => {
                if index == 0 {
                    self.x = value.x;
                } else {
                    self.y = value.y;
                }
            }
        }
    }

    fn title(&self) -> String {
        "Compose".to_string()
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

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo {
        let Connector { ty, label, .. } = self.inputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        if remotes.len() == 0 {
            ui.label("None");
        } else {
            let (r_index, remote_node) = &remotes[0];
            let new_value = remote_node.out_value(*r_index);

            self.in_value(index, new_value);

            ui.label(new_value.to_string());
        }

        T::get_node_pin(ty, connected)
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo {
        let Connector { ty, label, .. } = self.outputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        ui.label(self.values().to_string());

        T::get_node_pin(ty, connected)
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<T> {
        unimplemented!();
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> PinInfo {
        unimplemented!()
    }
}
