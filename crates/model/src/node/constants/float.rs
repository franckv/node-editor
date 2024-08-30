use std::marker::PhantomData;

use crate::node::{Connector, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FloatNode<T> {
    pub value: f32,
    #[serde(skip_serializing)]
    node_type: PhantomData<T>,
}

impl<T> Default for FloatNode<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 0] = [];
const OUTPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::F32,
    label: "x",
    editable: true,
}];

impl<T> NodeData<T> for FloatNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::F32(self.value)
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        &mut self.value
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "Float".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
        &OUTPUTS
    }
}
