use egui::{Color32, Pos2, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};

use crate::{
    graph::GraphView,
    node::{Connector, NodeValueType, NodeView},
};

pub struct NodeViewer;

impl NodeViewer {
    fn get_node_pin(ty: NodeValueType, connected: bool) -> PinInfo {
        let color = if connected {
            Color32::GREEN
        } else {
            Color32::RED
        };

        match ty {
            NodeValueType::F32 => PinInfo::circle().with_fill(color),
            NodeValueType::Vec2 => PinInfo::triangle().with_fill(color),
            NodeValueType::Vec3 => PinInfo::triangle().with_fill(color),
            NodeValueType::Any => PinInfo::star().with_fill(color),
            NodeValueType::None => unimplemented!(),
        }
    }
}

impl<T: NodeView<T> + GraphView<T> + Clone> SnarlViewer<T> for NodeViewer {
    fn title(&mut self, node: &T) -> String {
        node.title()
    }

    fn has_body(&mut self, node: &T) -> bool {
        node.has_body()
    }

    fn show_body(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<T>,
    ) {
        let inputs = inputs
            .iter()
            .flat_map(|pin| pin.remotes.iter().map(|remote| snarl[remote.node].clone()))
            .collect::<Vec<T>>();

        let node = &mut snarl[node];

        if node.has_body() {
            node.show_body(ui, &inputs);
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<T>) {
        let output = &snarl[from.id.node];
        let input = &snarl[to.id.node];

        let Connector { ty: out_type, .. } = &output.outputs()[from.id.output];
        let Connector { ty: in_type, .. } = &input.inputs()[to.id.input];

        if *in_type == NodeValueType::Any || *in_type == *out_type {
            for &remote in &to.remotes {
                snarl.disconnect(remote, to.id);
            }

            snarl.connect(from.id, to.id);
        }
    }

    fn outputs(&mut self, node: &T) -> usize {
        node.outputs().len()
    }

    fn inputs(&mut self, node: &T) -> usize {
        node.inputs().len()
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<T>,
    ) -> egui_snarl::ui::PinInfo {
        let remotes = pin
            .remotes
            .iter()
            .map(|remote| (remote.output, snarl[remote.node].clone()))
            .collect::<Vec<(usize, T)>>();

        let node = &mut snarl[pin.id.node];
        let index = pin.id.input;

        let Connector { ty, label, .. } = node.inputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        if remotes.len() == 0 {
            ui.label("None");
        } else {
            let (remote_index, remote_node) = &remotes[0];
            let remote_value = remote_node.out_value(*remote_index);

            node.in_value(index, remote_value);

            ui.label(remote_value.to_string());
        }

        Self::get_node_pin(ty, connected)
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<T>,
    ) -> egui_snarl::ui::PinInfo {
        let remotes = pin
            .remotes
            .iter()
            .map(|remote| (remote.input, snarl[remote.node].clone()))
            .collect::<Vec<(usize, T)>>();

        let node = &mut snarl[pin.id.node];
        let index = pin.id.output;

        let Connector {
            ty,
            label,
            editable,
        } = node.outputs()[index];
        let connected = remotes.len() > 0;

        ui.label(label);
        if editable && ty == NodeValueType::F32 {
            ui.add(egui::DragValue::new(node.f32_out_value_mut(index)));
        } else {
            ui.label(node.out_value(index).to_string());
        }

        Self::get_node_pin(ty, connected)
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<T>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, _scale: f32, snarl: &mut Snarl<T>) {
        ui.label("Add node");
        ui.separator();
        let node = T::show_graph_menu(ui);
        if let Some(node) = node {
            snarl.insert_node(pos, node);

            ui.close_menu();
        }
    }

    fn has_node_menu(&mut self, _node: &T) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<T>,
    ) {
        ui.label("Node menu");
        ui.separator();
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}
