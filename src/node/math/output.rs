use egui_snarl::ui::PinInfo;

use crate::node::math::MathNode;
use crate::node::NodeView;

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct OutputNode;

impl NodeView<MathNode> for OutputNode {
    fn title(&self) -> String {
        "Output".to_string()
    }

    fn inputs(&self) -> usize {
        1
    }

    fn outputs(&self) -> usize {
        0
    }

    fn connect(&self, _: &MathNode) -> bool {
        false
    }

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<MathNode>) {
        unimplemented!();
    }

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<(usize, MathNode)>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            MathNode::get_pin_float_disconnected()
        } else {
            match &remotes[0].1 {
                MathNode::Float(value) => {
                    ui.label(MathNode::format_float(value.value()));
                    MathNode::get_pin_float_connected()
                }
                MathNode::BinOp(value) => {
                    ui.label(MathNode::format_float(value.value()));
                    MathNode::get_pin_float_connected()
                }
                _ => unimplemented!(),
            }
        }
    }

    fn show_output(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, MathNode)>) -> PinInfo {
        unimplemented!();
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<MathNode> {
        unimplemented!();
    }
}
