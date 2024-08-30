use std::marker::PhantomData;

use crate::node::{Connector, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

impl Default for Ops {
    fn default() -> Self {
        Ops::Add
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BinOpNode<T> {
    pub op: Ops,
    pub a: f32,
    pub b: f32,
    #[serde(skip_serializing)]
    node_type: PhantomData<T>,
}

impl<T> Default for BinOpNode<T> {
    fn default() -> Self {
        Self {
            op: Default::default(),
            a: Default::default(),
            b: Default::default(),
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 2] = [
    Connector {
        ty: NodeValueType::F32,
        label: "a",
        editable: false,
    },
    Connector {
        ty: NodeValueType::F32,
        label: "b",
        editable: false,
    },
];
const OUTPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::F32,
    label: "result",
    editable: false,
}];

impl<T> NodeData<T> for BinOpNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        let value = match self.op {
            Ops::Add => self.a + self.b,
            Ops::Sub => self.a - self.b,
            Ops::Mul => self.a * self.b,
            Ops::Div => {
                if self.b == 0. {
                    0.
                } else {
                    self.a / self.b
                }
            }
        };

        NodeValue::F32(value)
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(value) => {
                if index == 0 {
                    self.a = value;
                } else {
                    self.b = value;
                }
            }
            _ => unimplemented!(),
        }
    }

    fn title(&self) -> String {
        "Binary Op".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
        &OUTPUTS
    }
}
