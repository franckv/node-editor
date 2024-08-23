use egui_snarl::ui::PinInfo;

use crate::node::math::Node;
use crate::node::{NodeValue, NodeValueType, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct OutputNode;

const INPUTS: [(NodeValueType, &str); 1] = [(NodeValueType::Any, "input")];
const OUTPUTS: [(NodeValueType, &str); 0] = [];

impl NodeView<Node> for OutputNode {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::None
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "Output".to_string()
    }

    fn inputs(&self) -> &[(NodeValueType, &str)] {
        &INPUTS
    }

    fn outputs(&self) -> &[(NodeValueType, &str)] {
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
            let new_value = remote.out_value(*r_index);

            ui.label(new_value.to_string());
            Node::get_pin_any_connected()
        }
    }

    fn show_output(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, Node)>) -> PinInfo {
        unimplemented!();
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!();
    }
}
