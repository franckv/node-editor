use egui_snarl::ui::PinInfo;

use crate::node::math::Node;
use crate::node::{NodeValueType, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct OutputNode;

const INPUTS: [NodeValueType; 1] = [NodeValueType::Any];
const OUTPUTS: [NodeValueType; 0] = [];

impl NodeView<Node> for OutputNode {
    fn title(&self) -> String {
        "Output".to_string()
    }

    fn inputs(&self) -> &[NodeValueType] {
        &INPUTS
    }

    fn outputs(&self) -> &[NodeValueType] {
        &OUTPUTS
    }

    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<Node>) {
        unimplemented!();
    }

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        _index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_any_disconnected()
        } else {
            let (r_index, remote) = &remotes[0];
            match remote {
                Node::Float(value) => {
                    ui.label(value.value().to_string());
                    Node::get_pin_any_connected()
                }
                Node::Vec2(value) => {
                    ui.label(value.value(*r_index).to_string());
                    Node::get_pin_any_connected()
                }
                Node::BinOp(value) => {
                    ui.label(value.value().to_string());
                    Node::get_pin_any_connected()
                }
                Node::Compose(value) => {
                    ui.label(value.values().to_string());
                    Node::get_pin_any_connected()
                }
                _ => unimplemented!(),
            }
        }
    }

    fn show_output(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, Node)>) -> PinInfo {
        unimplemented!();
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!();
    }
}
