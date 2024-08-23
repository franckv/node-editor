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

    pub fn has_body() -> bool {
        false
    }

    pub fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<Node>) {
        unimplemented!();
    }

    pub fn show_input(&mut self, ui: &mut egui::Ui, _index: usize, remotes: &Vec<Node>) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            match &remotes[0] {
                Node::Float(value) => {
                    ui.label(Node::format_float(value.value()));
                    Node::get_pin_float_connected()
                }
                Node::BinOp(value) => {
                    ui.label(Node::format_float(value.value()));
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
