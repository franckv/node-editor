use node_compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection};

use node_model::{
    vector::DecomposeNode, BinOpNode, ComposeNode, Connector, FloatNode, NodeData, NodeValue,
    OutputNode, VecNode, VertexInNode,
};

use crate::{graph::GraphView, view::NodeView};

type Node = FragmentShaderNode;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FragmentShaderNode {
    Output(OutputNode<Self>),
    Float(FloatNode<Self>),
    Vec(VecNode<Self>),
    BinOp(BinOpNode<Self>),
    Compose(ComposeNode<Self>),
    Decompose(DecomposeNode<Self>),
    VertexInput(VertexInNode<Self>),
}

impl NodeData<Node> for Node {
    fn out_value(&self, index: usize) -> NodeValue {
        match self {
            Node::Output(value) => value.out_value(index),
            Node::Float(value) => value.out_value(index),
            Node::Vec(value) => value.out_value(index),
            Node::BinOp(value) => value.out_value(index),
            Node::Compose(value) => value.out_value(index),
            Node::Decompose(value) => value.out_value(index),
            Node::VertexInput(value) => value.out_value(index),
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            Node::Output(value) => value.f32_out_value_mut(index),
            Node::Float(value) => value.f32_out_value_mut(index),
            Node::Vec(value) => value.f32_out_value_mut(index),
            Node::BinOp(value) => value.f32_out_value_mut(index),
            Node::Compose(value) => value.f32_out_value_mut(index),
            Node::Decompose(value) => value.f32_out_value_mut(index),
            Node::VertexInput(value) => value.f32_out_value_mut(index),
        }
    }

    fn in_value(&mut self, index: usize, new_value: NodeValue) {
        match self {
            Node::Output(value) => value.in_value(index, new_value),
            Node::Float(value) => value.in_value(index, new_value),
            Node::Vec(value) => value.in_value(index, new_value),
            Node::BinOp(value) => value.in_value(index, new_value),
            Node::Compose(value) => value.in_value(index, new_value),
            Node::Decompose(value) => value.in_value(index, new_value),
            Node::VertexInput(value) => value.in_value(index, new_value),
        }
    }

    fn title(&self) -> String {
        match self {
            Node::Output(value) => value.title(),
            Node::Float(value) => value.title(),
            Node::Vec(value) => value.title(),
            Node::BinOp(value) => value.title(),
            Node::Compose(value) => value.title(),
            Node::Decompose(value) => value.title(),
            Node::VertexInput(value) => value.title(),
        }
    }

    fn inputs(&self) -> &[Connector] {
        match self {
            Node::Output(value) => value.inputs(),
            Node::Float(value) => value.inputs(),
            Node::Vec(value) => value.inputs(),
            Node::BinOp(value) => value.inputs(),
            Node::Compose(value) => value.inputs(),
            Node::Decompose(value) => value.inputs(),
            Node::VertexInput(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> &[Connector] {
        match self {
            Node::Output(value) => value.outputs(),
            Node::Float(value) => value.outputs(),
            Node::Vec(value) => value.outputs(),
            Node::BinOp(value) => value.outputs(),
            Node::Compose(value) => value.outputs(),
            Node::Decompose(value) => value.outputs(),
            Node::VertexInput(value) => value.outputs(),
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
            Node::Decompose(value) => value.has_body(),
            Node::VertexInput(value) => value.has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<Node>) -> bool {
        match self {
            Node::Output(value) => value.show_body(ui, inputs),
            Node::Float(value) => value.show_body(ui, inputs),
            Node::Vec(value) => value.show_body(ui, inputs),
            Node::BinOp(value) => value.show_body(ui, inputs),
            Node::Compose(value) => value.show_body(ui, inputs),
            Node::Decompose(value) => value.show_body(ui, inputs),
            Node::VertexInput(value) => value.show_body(ui, inputs),
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
            if ui.button("Vec").clicked() {
                result = Some(Node::Vec(VecNode::default()));
            }
        });
        ui.menu_button("Inputs", |ui| {
            if ui.button("VertexInput").clicked() {
                result = Some(Node::VertexInput(VertexInNode::default()));
            }
        });
        if ui.button("Outputs").clicked() {
            result = Some(Node::Output(OutputNode::default()));
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(Node::BinOp(BinOpNode::default()));
            }
            if ui.button("Compose").clicked() {
                result = Some(Node::Compose(ComposeNode::default()));
            }
            if ui.button("Decompose").clicked() {
                result = Some(Node::Decompose(DecomposeNode::default()));
            }
        });

        result
    }
}

impl NodeCompile<Node, ShaderCompiler, ShaderSection> for Node {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        match self {
            Node::Output(value) => value.out_vars(id, index),
            Node::Float(value) => value.out_vars(id, index),
            Node::Vec(value) => value.out_vars(id, index),
            Node::BinOp(value) => value.out_vars(id, index),
            Node::Compose(value) => value.out_vars(id, index),
            Node::Decompose(value) => value.out_vars(id, index),
            Node::VertexInput(value) => value.out_vars(id, index),
        }
    }

    fn code(
        &self,
        id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        match self {
            Node::Output(value) => value.code(id, input_vars),
            Node::Float(value) => value.code(id, input_vars),
            Node::Vec(value) => value.code(id, input_vars),
            Node::BinOp(value) => value.code(id, input_vars),
            Node::Compose(value) => value.code(id, input_vars),
            Node::Decompose(value) => value.code(id, input_vars),
            Node::VertexInput(value) => value.code(id, input_vars),
        }
    }
}
