use std::marker::PhantomData;

use crate::node::{Connector, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Dim {
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VecNode<T> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub dim: Dim,
    #[serde(skip_serializing)]
    node_type: PhantomData<T>,
}

impl<T> Default for VecNode<T> {
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

const INPUTS: [Connector; 0] = [];
const OUTPUTS2: [Connector; 3] = [
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
const OUTPUTS3: [Connector; 4] = [
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
const OUTPUTS4: [Connector; 5] = [
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
        ty: NodeValueType::F32,
        label: "w",
        editable: true,
    },
    Connector {
        ty: NodeValueType::Vec4,
        label: "xyzw",
        editable: false,
    },
];

impl<T> NodeData<T> for VecNode<T> {
    fn out_value(&self, index: usize) -> NodeValue {
        match self.dim {
            Dim::Vec2 => {
                if index == 0 {
                    NodeValue::F32(self.x)
                } else if index == 1 {
                    NodeValue::F32(self.y)
                } else {
                    NodeValue::Vec2((self.x, self.y).into())
                }
            }
            Dim::Vec3 => {
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
            Dim::Vec4 => {
                if index == 0 {
                    NodeValue::F32(self.x)
                } else if index == 1 {
                    NodeValue::F32(self.y)
                } else if index == 2 {
                    NodeValue::F32(self.z)
                } else if index == 3 {
                    NodeValue::F32(self.w)
                } else {
                    NodeValue::Vec4((self.x, self.y, self.z, self.w).into())
                }
            }
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else if index == 2 && (self.dim == Dim::Vec3 || self.dim == Dim::Vec4) {
            &mut self.z
        } else if index == 3 && self.dim == Dim::Vec4 {
            &mut self.w
        } else {
            unimplemented!()
        }
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "Vec".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
        match self.dim {
            Dim::Vec2 => &OUTPUTS2,
            Dim::Vec3 => &OUTPUTS3,
            Dim::Vec4 => &OUTPUTS4,
        }
    }
}
