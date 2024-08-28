use std::marker::PhantomData;

use crate::{
    node::{Connector, NodeData, NodeValue, NodeValueType},
    Dim,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DecomposeNode<T> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub dim: Dim,
    node_type: PhantomData<T>,
}

impl<T> Default for DecomposeNode<T> {
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

const INPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::Any,
    label: "input",
    editable: false,
}];
const OUTPUTS2: [Connector; 2] = [
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
const OUTPUTS3: [Connector; 3] = [
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
];
const OUTPUTS4: [Connector; 4] = [
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

impl<T> NodeData<T> for DecomposeNode<T> {
    fn out_value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else if index == 2 && (self.dim == Dim::Vec3 || self.dim == Dim::Vec4) {
            NodeValue::F32(self.z)
        } else if index == 3 && self.dim == Dim::Vec4 {
            NodeValue::F32(self.w)
        } else {
            unimplemented!()
        }
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, _index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(x) => self.x = x,
            NodeValue::Vec2(vec2) => (self.x, self.y, self.dim) = (vec2.x, vec2.y, Dim::Vec2),
            NodeValue::Vec3(vec3) => {
                (self.x, self.y, self.z, self.dim) = (vec3.x, vec3.y, vec3.z, Dim::Vec3)
            }
            NodeValue::Vec4(vec4) => {
                (self.x, self.y, self.z, self.w, self.dim) =
                    (vec4.x, vec4.y, vec4.z, vec4.w, Dim::Vec4)
            }
            NodeValue::Mat4(_) => unimplemented!(),
            NodeValue::None => unimplemented!(),
        }
    }

    fn title(&self) -> String {
        "Decompose".to_string()
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
