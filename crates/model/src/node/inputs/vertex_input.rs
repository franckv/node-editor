use std::marker::PhantomData;

use glam::{Vec2, Vec3, Vec4};

use crate::node::{Connector, NodeData, NodeValue, NodeValueType};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VertexInNode<T> {
    pub color: Vec4,
    pub uv: Vec2,
    pub normal: Vec3,
    #[serde(skip_serializing)]
    node_type: PhantomData<T>,
}

impl<T> Default for VertexInNode<T> {
    fn default() -> Self {
        Self {
            color: Vec4::splat(1.),
            uv: Vec2::splat(1.),
            normal: Vec3::Z,
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 0] = [];
const OUTPUTS: [Connector; 3] = [
    Connector {
        ty: NodeValueType::Vec4,
        label: "color",
        editable: true,
    },
    Connector {
        ty: NodeValueType::Vec2,
        label: "uv",
        editable: true,
    },
    Connector {
        ty: NodeValueType::Vec3,
        label: "normal",
        editable: true,
    },
];

impl<T> NodeData<T> for VertexInNode<T> {
    fn out_value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::Vec4(self.color)
        } else if index == 1 {
            NodeValue::Vec2(self.uv)
        } else {
            NodeValue::Vec3(self.normal)
        }
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "VertexInput".to_string()
    }

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
        &OUTPUTS
    }
}
