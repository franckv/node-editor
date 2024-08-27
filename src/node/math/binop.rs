use std::marker::PhantomData;

use crate::{
    compiler::{NodeCompile, NodeParam},
    node::{Connector, NodeValue, NodeValueType, NodeView},
};

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
pub struct BinOpNode<T> {
    pub op: Ops,
    pub a: f32,
    pub b: f32,
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

impl<T> NodeView<T> for BinOpNode<T> {
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

    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        let old_op = self.op;

        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.op))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.op, Ops::Add, "Add");
                ui.selectable_value(&mut self.op, Ops::Sub, "Sub");
                ui.selectable_value(&mut self.op, Ops::Mul, "Mul");
                ui.selectable_value(&mut self.op, Ops::Div, "Div");
            });

        self.op != old_op
    }
}

impl<T> NodeCompile<T> for BinOpNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{:?}_{}", self.op, id).to_lowercase(),
            ty: OUTPUTS[index].ty,
        }
    }

    fn code(&self, id: usize, input_vars: &Vec<Option<NodeParam>>) -> String {
        let input_a = input_vars.get(0).expect("2 args");
        let input_b = input_vars.get(1).expect("2 args");

        let var = &self.out_vars(id, 0);

        let op = match self.op {
            Ops::Add => "+",
            Ops::Sub => "-",
            Ops::Mul => "*",
            Ops::Div => "/",
        };

        let code = if let (Some(a), Some(b)) = (input_a, input_b) {
            format!("float {} = {} {} {};", &var.name, &a.name, op, &b.name)
        } else {
            "".to_string()
        };

        code
    }
}
