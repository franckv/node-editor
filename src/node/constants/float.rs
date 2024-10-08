use std::marker::PhantomData;

use crate::{
    compiler::{NodeCompile, NodeParam},
    node::{Connector, NodeValue, NodeValueType, NodeView},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatNode<T> {
    value: f32,
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

impl<T> NodeView<T> for FloatNode<T> {
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

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!();
    }
}

impl<T> NodeCompile<T> for FloatNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("float_{}", id),
            ty: OUTPUTS[index].ty,
        }
    }

    fn code(&self, id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        let var = &self.out_vars(id, 0);

        let code = format!("float {} = {:.2};", &var.name, self.value);

        code
    }
}
