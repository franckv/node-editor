use egui::{Pos2, Ui};
use egui_snarl::{ui::SnarlViewer, InPin, NodeId, OutPin, Snarl};

use crate::node::NodeView;

pub struct NodeViewer;

impl<T: NodeView<T> + Clone> SnarlViewer<T> for NodeViewer {
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

        if output.connect(from.id.output, input, to.id.input) {
            for &remote in &to.remotes {
                snarl.disconnect(remote, to.id);
            }

            snarl.connect(from.id, to.id);
        }
    }

    fn outputs(&mut self, node: &T) -> usize {
        node.outputs()
    }

    fn inputs(&mut self, node: &T) -> usize {
        node.inputs()
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

        node.show_input(ui, pin.id.input, &remotes)
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

        node.show_output(ui, pin.id.output, &remotes)
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
