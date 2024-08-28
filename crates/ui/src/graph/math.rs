use node_compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection};

use node_model::{
    BinOpNode, ComposeNode, Connector, FloatNode, NodeData, NodeValue, OutputNode, VecNode,
};

use crate::{graph::GraphView, view::NodeView};

type Node = MathNode;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MathNode {
    Output(OutputNode<Self>),
    Float(FloatNode<Self>),
    Vec(VecNode<Self>),
    BinOp(BinOpNode<Self>),
    Compose(ComposeNode<Self>),
}

impl NodeData<Node> for Node {
    fn out_value(&self, index: usize) -> NodeValue {
        match self {
            Node::Output(value) => value.out_value(index),
            Node::Float(value) => value.out_value(index),
            Node::Vec(value) => value.out_value(index),
            Node::BinOp(value) => value.out_value(index),
            Node::Compose(value) => value.out_value(index),
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            Node::Output(value) => value.f32_out_value_mut(index),
            Node::Float(value) => value.f32_out_value_mut(index),
            Node::Vec(value) => value.f32_out_value_mut(index),
            Node::BinOp(value) => value.f32_out_value_mut(index),
            Node::Compose(value) => value.f32_out_value_mut(index),
        }
    }

    fn in_value(&mut self, index: usize, new_value: NodeValue) {
        match self {
            Node::Output(value) => value.in_value(index, new_value),
            Node::Float(value) => value.in_value(index, new_value),
            Node::Vec(value) => value.in_value(index, new_value),
            Node::BinOp(value) => value.in_value(index, new_value),
            Node::Compose(value) => value.in_value(index, new_value),
        }
    }

    fn title(&self) -> String {
        match self {
            Node::Output(value) => value.title(),
            Node::Float(value) => value.title(),
            Node::Vec(value) => value.title(),
            Node::BinOp(value) => value.title(),
            Node::Compose(value) => value.title(),
        }
    }

    fn inputs(&self) -> &[Connector] {
        match self {
            Node::Output(value) => value.inputs(),
            Node::Float(value) => value.inputs(),
            Node::Vec(value) => value.inputs(),
            Node::BinOp(value) => value.inputs(),
            Node::Compose(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> &[Connector] {
        match self {
            Node::Output(value) => value.outputs(),
            Node::Float(value) => value.outputs(),
            Node::Vec(value) => value.outputs(),
            Node::BinOp(value) => value.outputs(),
            Node::Compose(value) => value.outputs(),
        }
    }
}

impl NodeView<Node> for Node {
    fn has_body(&self) -> bool {
        match self {
            Node::Output(value) => value.has_body(),
            Node::Float(value) => value.has_body(),
            Node::Vec(value) => value.has_body(),
            Node::BinOp(value) => value.has_body(),
            Node::Compose(value) => value.has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<Node>) -> bool {
        match self {
            Node::Output(value) => value.show_body(ui, inputs),
            Node::Float(value) => value.show_body(ui, inputs),
            Node::Vec(value) => value.show_body(ui, inputs),
            Node::BinOp(value) => value.show_body(ui, inputs),
            Node::Compose(value) => value.show_body(ui, inputs),
        }
    }
}

impl GraphView<Node> for Node {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<Node> {
        let mut result = None;
        ui.menu_button("Constants", |ui| {
            if ui.button("Float").clicked() {
                result = Some(Node::Float(FloatNode::default()));
            }
            if ui.button("Vec2").clicked() {
                result = Some(Node::Vec(VecNode::default()));
            }
        });
        if ui.button("Output").clicked() {
            result = Some(Node::Output(OutputNode::default()));
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(Node::BinOp(BinOpNode::default()));
            }
            if ui.button("Compose").clicked() {
                result = Some(Node::Compose(ComposeNode::default()));
            }
        });

        result
    }
}

impl NodeCompile<Node, ShaderCompiler, ShaderSection> for Node {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        match self {
            MathNode::Output(value) => value.out_vars(id, index),
            MathNode::Float(value) => value.out_vars(id, index),
            MathNode::Vec(value) => value.out_vars(id, index),
            MathNode::BinOp(value) => value.out_vars(id, index),
            MathNode::Compose(value) => value.out_vars(id, index),
        }
    }

    fn code(
        &self,
        id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        match self {
            MathNode::Output(value) => value.code(id, input_vars),
            MathNode::Float(value) => value.code(id, input_vars),
            MathNode::Vec(value) => value.code(id, input_vars),
            MathNode::BinOp(value) => value.code(id, input_vars),
            MathNode::Compose(value) => value.code(id, input_vars),
        }
    }
}
