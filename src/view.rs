use egui::{Pos2, Ui};
use egui_snarl::{ui::SnarlViewer, InPin, NodeId, OutPin, Snarl};

use crate::node::{BinOpNode, FloatNode, Node, OutputNode};

pub struct NodeViewer;

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        node.title()
    }

    fn has_body(&mut self, node: &Node) -> bool {
        node.has_body()
    }

    fn show_body(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        let inputs = inputs
            .iter()
            .flat_map(|pin| pin.remotes.iter().map(|remote| snarl[remote.node].clone()))
            .collect::<Vec<Node>>();

        let node = &mut snarl[node];

        if node.has_body() {
            node.show_body(ui, &inputs);
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let output = &snarl[from.id.node];
        let input = &snarl[to.id.node];

        if output.connect(input) {
            for &remote in &to.remotes {
                snarl.disconnect(remote, to.id);
            }

            snarl.connect(from.id, to.id);
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        node.outputs()
    }

    fn inputs(&mut self, node: &Node) -> usize {
        node.inputs()
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> egui_snarl::ui::PinInfo {
        let remotes = pin
            .remotes
            .iter()
            .map(|remote| snarl[remote.node].clone())
            .collect::<Vec<Node>>();

        let node = &mut snarl[pin.id.node];

        node.show_input(ui, pin.id.input, &remotes)
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> egui_snarl::ui::PinInfo {
        let remotes = pin
            .remotes
            .iter()
            .map(|remote| snarl[remote.node].clone())
            .collect::<Vec<Node>>();

        let node = &mut snarl[pin.id.node];

        node.show_output(ui, &remotes)
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, _scale: f32, snarl: &mut Snarl<Node>) {
        ui.label("Add node");
        ui.separator();
        if ui.button("Float").clicked() {
            snarl.insert_node(pos, Node::Float(FloatNode::default()));
            ui.close_menu();
        }
        if ui.button("Output").clicked() {
            snarl.insert_node(pos, Node::Output(OutputNode::default()));
            ui.close_menu();
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("BinOp").clicked() {
                snarl.insert_node(pos, Node::BinOp(BinOpNode::default()));
                ui.close_menu();
            }
        });
    }

    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.label("Node menu");
        ui.separator();
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}
