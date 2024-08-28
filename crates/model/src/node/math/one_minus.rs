use std::marker::PhantomData;

use crate::node::{Connector, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OneMinusNode<T> {
    pub x: f32,
    node_type: PhantomData<T>,
}

impl<T> Default for OneMinusNode<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::F32,
    label: "x",
    editable: false,
}];
const OUTPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::F32,
    label: "result",
    editable: false,
}];

impl<T> NodeData<T> for OneMinusNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::F32(1. - self.x)
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(value) => {
                if index == 0 {
                    self.x = value;
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }

    fn title(&self) -> String {
        "One Minus".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
        &OUTPUTS
    }
}
