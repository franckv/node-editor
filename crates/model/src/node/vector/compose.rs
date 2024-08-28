use std::marker::PhantomData;

use crate::node::{Connector, Dim, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComposeNode<T> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub dim: Dim,
    node_type: PhantomData<T>,
}

impl<T> Default for ComposeNode<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
            dim: Dim::Vec2,
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 4] = [
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
    Connector {
        ty: NodeValueType::F32,
        label: "z",
        editable: false,
    },
    Connector {
        ty: NodeValueType::F32,
        label: "w",
        editable: false,
    },
];
const OUTPUTS: [Connector; 3] = [
    Connector {
        ty: NodeValueType::Vec2,
        label: "xy",
        editable: false,
    },
    Connector {
        ty: NodeValueType::Vec3,
        label: "xyz",
        editable: false,
    },
    Connector {
        ty: NodeValueType::Vec4,
        label: "xyzw",
        editable: false,
    },
];

impl<T> NodeData<T> for ComposeNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        match self.dim {
            Dim::Vec2 => NodeValue::Vec2((self.x, self.y).into()),
            Dim::Vec3 => NodeValue::Vec3((self.x, self.y, self.z).into()),
            Dim::Vec4 => NodeValue::Vec4((self.x, self.y, self.z, self.w).into()),
        }
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(value) => {
                if index == 0 {
                    self.x = value;
                } else if index == 1 {
                    self.y = value;
                } else if index == 2 {
                    self.z = value;
                } else if index == 3 {
                    self.w = value;
                }
            }
            _ => unimplemented!(),
        }
    }

    fn title(&self) -> String {
        "Compose".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        match self.dim {
            Dim::Vec2 => &INPUTS[0..2],
            Dim::Vec3 => &INPUTS[0..3],
            Dim::Vec4 => &INPUTS,
        }
    }

    fn outputs(&self) -> &[Connector] {
        match self.dim {
            Dim::Vec2 => &OUTPUTS[0..1],
            Dim::Vec3 => &OUTPUTS[1..2],
            Dim::Vec4 => &OUTPUTS[2..3],
        }
    }
}
