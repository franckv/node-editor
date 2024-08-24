use std::marker::PhantomData;

use egui_snarl::ui::PinInfo;

use crate::node::{NodeValue, NodeValueType, NodeView};

#[derive(Clone, Debug, serde::Serialize)]
pub struct FloatNode<T> {
    value: f32,
    node_type: PhantomData<T>,
}

impl<T> Default for FloatNode<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            node_type: Default::default(),
        }
    }
}

impl<T> FloatNode<T> {
    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

const INPUTS: [(NodeValueType, &str); 0] = [];
const OUTPUTS: [(NodeValueType, &str); 1] = [(NodeValueType::F32, "x")];

impl<T: NodeView<T>> NodeView<T> for FloatNode<T> {
    fn out_value(&self, _index: usize) -> NodeValue {
        NodeValue::F32(self.value)
    }

    fn in_value(&mut self, _index: usize, _value: NodeValue) {
        unimplemented!()
    }

    fn title(&self) -> String {
        "Float".to_string()
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

    fn show_input(&mut self, _: &mut egui::Ui, _: usize, _: &Vec<(usize, T)>) -> PinInfo {
        unimplemented!();
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo {
        let (ty, label) = self.outputs()[index];
        ui.label(label);
        ui.add(egui::DragValue::new(self.value_mut()));
        if remotes.len() > 0 {
            T::get_node_pin(ty, true)
        } else {
            T::get_node_pin(ty, false)
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<T> {
        unimplemented!();
    }

    fn get_node_pin(_ty: NodeValueType, _connected: bool) -> PinInfo {
        unimplemented!()
    }
}
