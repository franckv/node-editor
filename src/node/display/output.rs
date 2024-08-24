use std::marker::PhantomData;

use egui_snarl::ui::PinInfo;

use crate::node::{Connector, NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

const INPUTS: [Connector; 1] = [Connector {
    ty: NodeValueType::Any,
    label: "input",
    editable: false,
}];
const OUTPUTS: [Connector; 0] = [];

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

    fn inputs(&self) -> &[Connector] {
        &INPUTS
    }

    fn outputs(&self) -> &[Connector] {
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
        let Connector { ty, label, .. } = self.inputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        if remotes.len() == 0 {
            ui.label("None");
        } else {
            let (r_index, remote) = &remotes[0];
            let new_value = remote.out_value(*r_index);

            ui.label(new_value.to_string());
        }

        T::get_node_pin(ty, connected)
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
