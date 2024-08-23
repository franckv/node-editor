use egui_snarl::ui::PinInfo;

use crate::node::math::MathNode;
use crate::node::NodeView;

#[derive(Clone, Default, Debug)]
pub struct OutputNode;

impl OutputNode {
    pub fn title() -> String {
        "Output".to_string()
    }

    pub fn inputs() -> usize {
        1
    }

    pub fn outputs() -> usize {
        0
    }

    pub fn connect(_: &MathNode) -> bool {
        false
    }

    pub fn has_body() -> bool {
        false
    }

    pub fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<MathNode>) {
        unimplemented!();
    }

    pub fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<MathNode>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            MathNode::get_pin_float_disconnected()
        } else {
            match &remotes[0] {
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

    pub fn show_output(&mut self, _: &mut egui::Ui, _: &Vec<MathNode>) -> PinInfo {
        unimplemented!();
    }
}
