use std::marker::PhantomData;

use egui_snarl::ui::PinInfo;

use crate::node::{NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize)]
pub struct OutputNode<T> {
    node_type: PhantomData<T>,
}

impl<T> Default for OutputNode<T> {
    fn default() -> Self {
        Self {
            node_type: Default::default(),
        }
    }
}

const INPUTS: [(NodeValueType, &str); 1] = [(NodeValueType::Any, "input")];
const OUTPUTS: [(NodeValueType, &str); 0] = [];

impl<T: NodeView<T>> NodeView<T> for OutputNode<T> {
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

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) {
        unimplemented!();
    }

    fn show_input(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo {
        let (ty, label) = self.inputs()[index];
        ui.label(label);
        if remotes.len() == 0 {
            ui.label("None");
            T::get_node_pin(ty, false)
        } else {
            let (r_index, remote) = &remotes[0];
            let new_value = remote.out_value(*r_index);

            ui.label(new_value.to_string());
            T::get_node_pin(ty, true)
        }
    }

    fn show_output(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, T)>) -> PinInfo {
        unimplemented!();
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<T> {
        unimplemented!();
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> PinInfo {
        unimplemented!()
    }
}
