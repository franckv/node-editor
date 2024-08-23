use camera::CameraPositionNode;
use egui::Color32;
use egui_snarl::ui::PinInfo;

mod binop;
mod camera;
mod float;

pub use binop::BinOpNode;
pub use float::FloatNode;

use super::NodeView;

#[derive(Clone, Debug, serde::Serialize)]
pub enum FragmentNode {
    Float(FloatNode),
    BinOp(BinOpNode),
    CameraPosition(CameraPositionNode),
}

impl FragmentNode {
    fn get_pin_float_disconnected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::RED)
    }

    fn get_pin_float_connected() -> PinInfo {
        PinInfo::circle().with_fill(Color32::GREEN)
    }
}

impl NodeView<FragmentNode> for FragmentNode {
    fn title(&self) -> String {
        match self {
            FragmentNode::Float(value) => value.title(),
            FragmentNode::BinOp(value) => value.title(),
            FragmentNode::CameraPosition(value) => value.title(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            FragmentNode::Float(value) => value.inputs(),
            FragmentNode::BinOp(value) => value.inputs(),
            FragmentNode::CameraPosition(value) => value.inputs(),
        }
    }

    fn outputs(&self) -> usize {
        match self {
            FragmentNode::Float(value) => value.outputs(),
            FragmentNode::BinOp(value) => value.outputs(),
            FragmentNode::CameraPosition(value) => value.outputs(),
        }
    }

    fn connect(&self, other: &FragmentNode) -> bool {
        match self {
            FragmentNode::Float(value) => value.connect(other),
            FragmentNode::BinOp(value) => value.connect(other),
            FragmentNode::CameraPosition(value) => value.connect(other),
        }
    }

    fn has_body(&self) -> bool {
        match self {
            FragmentNode::Float(value) => value.has_body(),
            FragmentNode::BinOp(value) => value.has_body(),
            FragmentNode::CameraPosition(value) => value.has_body(),
        }
    }

    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<FragmentNode>) {
        match self {
            FragmentNode::Float(value) => value.show_body(ui, inputs),
            FragmentNode::BinOp(value) => value.show_body(ui, inputs),
            FragmentNode::CameraPosition(value) => value.show_body(ui, inputs),
        }
    }

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, FragmentNode)>,
    ) -> PinInfo {
        match self {
            FragmentNode::Float(value) => value.show_input(ui, index, remotes),
            FragmentNode::BinOp(value) => value.show_input(ui, index, remotes),
            FragmentNode::CameraPosition(value) => value.show_input(ui, index, remotes),
        }
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, FragmentNode)>,
    ) -> PinInfo {
        match self {
            FragmentNode::Float(value) => value.show_output(ui, index, remotes),
            FragmentNode::BinOp(value) => value.show_output(ui, index, remotes),
            FragmentNode::CameraPosition(value) => value.show_output(ui, index, remotes),
        }
    }

    fn show_graph_menu(ui: &mut egui::Ui) -> Option<FragmentNode> {
        let mut result = None;
        if ui.button("Float").clicked() {
            result = Some(FragmentNode::Float(FloatNode::default()));
        }
        ui.menu_button("Inputs", |ui| {
            if ui.button("CameraPos").clicked() {
                result = Some(FragmentNode::CameraPosition(CameraPositionNode::default()));
            }
        });
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                result = Some(FragmentNode::BinOp(BinOpNode::default()));
            }
        });

        result
    }
}

impl FragmentNode {
    pub fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }
}
