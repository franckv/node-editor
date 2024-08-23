use egui::Color32;
use egui_snarl::ui::PinInfo;

mod binop;
mod camera;
mod float;

pub use binop::BinOpNode;
pub use float::FloatNode;

use crate::node::NodeView;
use camera::CameraPositionNode;

type Node = FragmentNode;

#[derive(Clone, Debug, serde::Serialize)]
pub enum FragmentNode {
    Float(FloatNode),
    BinOp(BinOpNode),
    CameraPosition(CameraPositionNode),
}

impl Node {
    fn get_pin_float_disconnected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::RED)
    }

    fn get_pin_float_connected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::GREEN)
    }

    fn get_pin_vec_disconnected() -> PinInfo {
        PinInfo::triangle().with_fill(Color32::RED)
    }

    fn get_pin_vec_connected() -> PinInfo {
        PinInfo::triangle().with_fill(Color32::GREEN)
    }

    fn _get_pin_any_disconnected() -> PinInfo {
        PinInfo::star().with_fill(Color32::RED)
    }

    fn _get_pin_any_connected() -> PinInfo {
        PinInfo::star().with_fill(Color32::GREEN)
    }
}

impl NodeView<Node> for Node {
    fn title(&self) -> String {
        match self {
            Node::Float(value) => value.title(),
            Node::BinOp(value) => value.title(),
            Node::CameraPosition(value) => value.title(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            Node::Float(value) => value.inputs(),
            Node::BinOp(value) => value.inputs(),
            Node::CameraPosition(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> usize {
        match self {
            Node::Float(value) => value.outputs(),
            Node::BinOp(value) => value.outputs(),
            Node::CameraPosition(value) => value.outputs(),
        }
    }

    fn connect(&self, index: usize, other: &Node, other_index: usize) -> bool {
        match self {
            Node::Float(value) => value.connect(index, other, other_index),
            Node::BinOp(value) => value.connect(index, other, other_index),
            Node::CameraPosition(value) => value.connect(index, other, other_index),
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

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        match self {
            Node::Float(value) => value.show_input(ui, index, remotes),
            Node::BinOp(value) => value.show_input(ui, index, remotes),
            Node::CameraPosition(value) => value.show_input(ui, index, remotes),
        }
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        match self {
            Node::Float(value) => value.show_output(ui, index, remotes),
            Node::BinOp(value) => value.show_output(ui, index, remotes),
            Node::CameraPosition(value) => value.show_output(ui, index, remotes),
        }
    }

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
