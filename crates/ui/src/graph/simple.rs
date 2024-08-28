use node_compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection};

use node_model::{BinOpNode, Connector, FloatNode, NodeData, NodeValue};

use crate::{graph::GraphView, view::NodeView};

type Node = SimpleNode;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SimpleNode {
    Float(FloatNode<Self>),
    BinOp(BinOpNode<Self>),
}

impl NodeData<Node> for Node {
    fn out_value(&self, index: usize) -> NodeValue {
        match self {
            Node::Float(value) => value.out_value(index),
            Node::BinOp(value) => value.out_value(index),
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            Node::Float(value) => value.f32_out_value_mut(index),
            Node::BinOp(value) => value.f32_out_value_mut(index),
        }
    }

    fn in_value(&mut self, index: usize, new_value: NodeValue) {
        match self {
            Node::Float(value) => value.in_value(index, new_value),
            Node::BinOp(value) => value.in_value(index, new_value),
        }
    }

    fn title(&self) -> String {
        match self {
            Node::Float(value) => value.title(),
            Node::BinOp(value) => value.title(),
        }
    }

    fn inputs(&self) -> &[Connector] {
        match self {
            Node::Float(value) => value.inputs(),
            Node::BinOp(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> &[Connector] {
        match self {
            Node::Float(value) => value.outputs(),
            Node::BinOp(value) => value.outputs(),
        }
    }
}

impl NodeView<Node> for Node {
    fn has_body(&self) -> bool {
        match self {
            Node::Float(value) => value.has_body(),
            Node::BinOp(value) => value.has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<Node>) -> bool {
        match self {
            Node::Float(value) => value.show_body(ui, inputs),
            Node::BinOp(value) => value.show_body(ui, inputs),
        }
    }
}

impl GraphView<Node> for Node {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<Node> {
        let mut result = None;
        if ui.button("Float").clicked() {
            result = Some(Node::Float(FloatNode::default()));
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(Node::BinOp(BinOpNode::default()));
            }
        });

        result
    }
}

impl NodeCompile<Node, ShaderCompiler, ShaderSection> for Node {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        match self {
            Node::Float(value) => value.out_vars(id, index),
            Node::BinOp(value) => value.out_vars(id, index),
        }
    }

    fn code(
        &self,
        id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        match self {
            Node::Float(value) => value.code(id, input_vars),
            Node::BinOp(value) => value.code(id, input_vars),
        }
    }
}
