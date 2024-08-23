use egui_snarl::ui::PinInfo;

use super::Node;

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

    pub fn connect(_: &Node) -> bool {
        false
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, _index: usize, remotes: &Vec<Node>) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            match &remotes[0] {
                Node::Float(value) => {
                    ui.label(format!("{}", value.value()));
                    Node::get_pin_float_connected()
                }
                Node::OpAdd(value) => {
                    ui.label(format!("{}", value.value()));
                    Node::get_pin_float_connected()
                }
                Node::OpSub(value) => {
                    ui.label(format!("{}", value.value()));
                    Node::get_pin_float_connected()
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn show_output(&mut self, _: &mut egui::Ui, _: &Vec<Node>) -> PinInfo {
        unimplemented!();
    }
}
