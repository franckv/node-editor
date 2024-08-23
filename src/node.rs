use egui::Color32;
use egui_snarl::ui::PinInfo;

mod binop;
mod float;
mod output;

pub use binop::BinOpNode;
pub use float::FloatNode;
pub use output::OutputNode;

#[derive(Clone, Debug)]
pub enum Node {
    Output(OutputNode),
    Float(FloatNode),
    BinOp(BinOpNode),
}

impl Node {
    pub fn title(&self) -> String {
        match self {
            Node::Output(_) => OutputNode::title(),
            Node::Float(_) => FloatNode::title(),
            Node::BinOp(_) => BinOpNode::title(),
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Node::Output(_) => OutputNode::inputs(),
            Node::Float(_) => FloatNode::inputs(),
            Node::BinOp(_) => BinOpNode::inputs(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Node::Output(_) => OutputNode::outputs(),
            Node::Float(_) => FloatNode::outputs(),
            Node::BinOp(_) => BinOpNode::outputs(),
        }
    }

    pub fn connect(&self, other: &Node) -> bool {
        match self {
            Node::Output(_) => OutputNode::connect(other),
            Node::Float(_) => FloatNode::connect(other),
            Node::BinOp(_) => BinOpNode::connect(other),
        }
    }

    pub fn has_body(&self) -> bool {
        match self {
            Node::Output(_) => OutputNode::has_body(),
            Node::Float(_) => FloatNode::has_body(),
            Node::BinOp(_) => BinOpNode::has_body(),
        }
    }

    pub fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<Node>) {
        match self {
            Node::Output(value) => value.show_body(ui, inputs),
            Node::Float(value) => value.show_body(ui, inputs),
            Node::BinOp(value) => value.show_body(ui, inputs),
        }
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<Node>) -> PinInfo {
        match self {
            Node::Output(value) => value.show_input(ui, index, remotes),
            Node::Float(value) => value.show_input(ui, index, remotes),
            Node::BinOp(value) => value.show_input(ui, index, remotes),
        }
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<Node>) -> PinInfo {
        match self {
            Node::Output(value) => value.show_output(ui, remotes),
            Node::Float(value) => value.show_output(ui, remotes),
            Node::BinOp(value) => value.show_output(ui, remotes),
        }
    }

    pub fn get_pin_float_disconnected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::RED)
    }

    pub fn get_pin_float_connected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::GREEN)
    }

    pub fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }
}
