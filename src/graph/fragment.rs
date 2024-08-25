use crate::graph::GraphView;
use crate::node::{BinOpNode, CameraPositionNode, Connector, FloatNode};
use crate::node::{NodeValue, NodeView};

type Node = FragmentNode;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FragmentNode {
    Float(FloatNode<Self>),
    BinOp(BinOpNode<Self>),
    CameraPosition(CameraPositionNode<Self>),
}

impl NodeView<Node> for Node {
    fn out_value(&self, index: usize) -> NodeValue {
        match self {
            Node::Float(value) => value.out_value(index),
            Node::BinOp(value) => value.out_value(index),
            Node::CameraPosition(value) => value.out_value(index),
        }
    }

    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32 {
        match self {
            Node::Float(value) => value.f32_out_value_mut(index),
            Node::BinOp(value) => value.f32_out_value_mut(index),
            Node::CameraPosition(value) => value.f32_out_value_mut(index),
        }
    }

    fn in_value(&mut self, index: usize, new_value: NodeValue) {
        match self {
            Node::Float(value) => value.in_value(index, new_value),
            Node::BinOp(value) => value.in_value(index, new_value),
            Node::CameraPosition(value) => value.in_value(index, new_value),
        }
    }

    fn title(&self) -> String {
        match self {
            Node::Float(value) => value.title(),
            Node::BinOp(value) => value.title(),
            Node::CameraPosition(value) => value.title(),
        }
    }

    fn inputs(&self) -> &[Connector] {
        match self {
            Node::Float(value) => value.inputs(),
            Node::BinOp(value) => value.inputs(),
            Node::CameraPosition(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> &[Connector] {
        match self {
            Node::Float(value) => value.outputs(),
            Node::BinOp(value) => value.outputs(),
            Node::CameraPosition(value) => value.outputs(),
        }
    }

    fn has_body(&self) -> bool {
        match self {
            Node::Float(value) => value.has_body(),
            Node::BinOp(value) => value.has_body(),
            Node::CameraPosition(value) => value.has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<Node>) {
        match self {
            Node::Float(value) => value.show_body(ui, inputs),
            Node::BinOp(value) => value.show_body(ui, inputs),
            Node::CameraPosition(value) => value.show_body(ui, inputs),
        }
    }
}

impl GraphView<Node> for Node {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<Node> {
        let mut result = None;
        if ui.button("Float").clicked() {
            result = Some(Node::Float(FloatNode::default()));
        }
        ui.menu_button("Inputs", |ui| {
            if ui.button("CameraPos").clicked() {
                result = Some(Node::CameraPosition(CameraPositionNode::default()));
            }
        });
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(Node::BinOp(BinOpNode::default()));
            }
        });

        result
    }
}

impl Node {
    pub fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }
}
