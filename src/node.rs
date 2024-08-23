use egui::Color32;
use egui_snarl::ui::PinInfo;

mod float;
mod op_add;
mod output;

pub use float::FloatNode;
pub use op_add::AddNode;
pub use output::OutputNode;

#[derive(Clone, Debug)]
pub enum Node {
    Output(OutputNode),
    Float(FloatNode),
    OpAdd(AddNode),
}

impl Node {
    pub fn title(&self) -> String {
        match self {
            Node::Output(_) => OutputNode::title(),
            Node::Float(_) => FloatNode::title(),
            Node::OpAdd(_) => AddNode::title(),
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Node::Output(_) => OutputNode::inputs(),
            Node::Float(_) => FloatNode::inputs(),
            Node::OpAdd(_) => AddNode::inputs(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Node::Output(_) => OutputNode::outputs(),
            Node::Float(_) => FloatNode::outputs(),
            Node::OpAdd(_) => AddNode::outputs(),
        }
    }

    pub fn connect(&self, other: &Node) -> bool {
        match self {
            Node::Output(_) => OutputNode::connect(other),
            Node::Float(_) => FloatNode::connect(other),
            Node::OpAdd(_) => AddNode::connect(other),
        }
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<Node>) -> PinInfo {
        match self {
            Node::Output(value) => value.show_input(ui, index, remotes),
            Node::Float(value) => value.show_input(ui, index, remotes),
            Node::OpAdd(value) => value.show_input(ui, index, remotes),
        }
    }

    pub fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<Node>) -> PinInfo {
        match self {
            Node::Output(value) => value.show_output(ui, remotes),
            Node::Float(value) => value.show_output(ui, remotes),
            Node::OpAdd(value) => value.show_output(ui, remotes),
        }
    }

    pub fn get_pin_float_disconnected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::RED)
    }

    pub fn get_pin_float_connected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::GREEN)
    }
}
