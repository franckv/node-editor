use std::marker::PhantomData;

use crate::{
    compiler::{NodeCompile, NodeParam},
    node::{Connector, NodeValue, NodeValueType, NodeView},
};

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

impl<T> NodeView<T> for Vec2Node<T> {
    fn out_value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else {
            NodeValue::Vec2((self.x, self.y).into())
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
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
}

const VAR_NAME: &str = "vec_";

impl<T> NodeCompile<T> for Vec2Node<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{}{}.{}", VAR_NAME, id, OUTPUTS[index].label),
            ty: OUTPUTS[index].ty,
        }
    }

    fn code(&self, id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        let var = format!("{}{}", VAR_NAME, id);

        format!("vec2 {} = vec2({:.2}, {:.2});", var, self.x, self.y)
    }
}
