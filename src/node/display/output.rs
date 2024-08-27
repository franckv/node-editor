use std::marker::PhantomData;

use crate::{
    compiler::{NodeCompile, NodeParam},
    node::{Connector, NodeValue, NodeValueType, NodeView},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutputNode<T> {
    node_type: PhantomData<T>,
}

impl<T> Default for OutputNode<T> {
    fn default() -> Self {
        Self {
            node_type: Default::default(),
        }
    }
}

const INPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::Any,
    label: "input",
    editable: false,
}];
const OUTPUTS: [Connector; 0] = [];

impl<T> NodeView<T> for OutputNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::None
    }

    fn f32_out_value_mut(&mut self, _index: usize) -> &mut f32 {
        unimplemented!()
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {}

    fn title(&self) -> String {
        "Output".to_string()
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

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!();
    }
}

impl<T> NodeCompile<T> for OutputNode<T> {
    fn out_vars(&self, _id: usize, _index: usize) -> NodeParam {
        unimplemented!()
    }

    fn code(&self, _id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        "".to_string()
    }
}
