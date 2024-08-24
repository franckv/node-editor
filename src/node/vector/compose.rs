use std::marker::PhantomData;

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

impl<T> NodeView<T> for ComposeNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::Vec2((self.x, self.y).into())
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
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
}
