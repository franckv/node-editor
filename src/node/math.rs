use egui::Color32;
use egui_snarl::ui::PinInfo;

mod binop;
mod float;
mod output;

pub use binop::BinOpNode;
pub use float::FloatNode;
pub use output::OutputNode;

use super::NodeView;

#[derive(Clone, Debug)]
pub enum MathNode {
    Output(OutputNode),
    Float(FloatNode),
    BinOp(BinOpNode),
}

impl NodeView<MathNode> for MathNode {
    fn title(&self) -> String {
        match self {
            MathNode::Output(_) => OutputNode::title(),
            MathNode::Float(_) => FloatNode::title(),
            MathNode::BinOp(_) => BinOpNode::title(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            MathNode::Output(_) => OutputNode::inputs(),
            MathNode::Float(_) => FloatNode::inputs(),
            MathNode::BinOp(_) => BinOpNode::inputs(),
        }
    }

    fn outputs(&self) -> usize {
        match self {
            MathNode::Output(_) => OutputNode::outputs(),
            MathNode::Float(_) => FloatNode::outputs(),
            MathNode::BinOp(_) => BinOpNode::outputs(),
        }
    }

    fn connect(&self, other: &MathNode) -> bool {
        match self {
            MathNode::Output(_) => OutputNode::connect(other),
            MathNode::Float(_) => FloatNode::connect(other),
            MathNode::BinOp(_) => BinOpNode::connect(other),
        }
    }

    fn has_body(&self) -> bool {
        match self {
            MathNode::Output(_) => OutputNode::has_body(),
            MathNode::Float(_) => FloatNode::has_body(),
            MathNode::BinOp(_) => BinOpNode::has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<MathNode>) {
        match self {
            MathNode::Output(value) => value.show_body(ui, inputs),
            MathNode::Float(value) => value.show_body(ui, inputs),
            MathNode::BinOp(value) => value.show_body(ui, inputs),
        }
    }

    fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<MathNode>) -> PinInfo {
        match self {
            MathNode::Output(value) => value.show_input(ui, index, remotes),
            MathNode::Float(value) => value.show_input(ui, index, remotes),
            MathNode::BinOp(value) => value.show_input(ui, index, remotes),
        }
    }

    fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<MathNode>) -> PinInfo {
        match self {
            MathNode::Output(value) => value.show_output(ui, remotes),
            MathNode::Float(value) => value.show_output(ui, remotes),
            MathNode::BinOp(value) => value.show_output(ui, remotes),
        }
    }

    fn get_pin_float_disconnected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::RED)
    }

    fn get_pin_float_connected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::GREEN)
    }

    fn show_graph_menu(ui: &mut egui::Ui) -> Option<MathNode> {
        let mut result = None;
        if ui.button("Float").clicked() {
            result = Some(MathNode::Float(FloatNode::default()));
        }
        if ui.button("Output").clicked() {
            result = Some(MathNode::Output(OutputNode::default()));
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(MathNode::BinOp(BinOpNode::default()));
            }
        });

        result
    }
}

impl MathNode {
    pub fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }
}
