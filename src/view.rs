use egui::{Color32, Pos2, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};

use crate::node::{AddNode, Node};

pub struct NodeViewer;

impl SnarlViewer<Node> for NodeViewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Output() => "Output".to_string(),
            Node::Float(_) => "Float".to_string(),
            Node::OpAdd(_) => "Add".to_string(),
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        tracing::info!("connect");
        let output = &snarl[from.id.node];
        let input = &snarl[to.id.node];

        match (output, input) {
            (Node::Float(_), Node::Output()) => {}
            (Node::Float(_), Node::OpAdd(_)) => {}
            (Node::OpAdd(_), Node::Output()) => {}
            (Node::OpAdd(_), Node::OpAdd(_)) => {}
            (_, _) => unimplemented!(),
        }

        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }

        snarl.connect(from.id, to.id);
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Output() => 0,
            Node::Float(_) => 1,
            Node::OpAdd(_) => 1,
        }
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Output() => 1,
            Node::Float(_) => 0,
            Node::OpAdd(_) => 2,
        }
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> egui_snarl::ui::PinInfo {
        match &snarl[pin.id.node] {
            Node::Output() => match &*pin.remotes {
                [] => {
                    ui.label("None");
                    PinInfo::square().with_fill(Color32::RED)
                }
                [remote] => match &snarl[remote.node] {
                    Node::Float(value) => {
                        ui.label(format!("{}", value));
                        PinInfo::circle().with_fill(Color32::GREEN)
                    }
                    Node::OpAdd(value) => {
                        ui.label(format!("{}", value.value()));
                        PinInfo::circle().with_fill(Color32::GREEN)
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            Node::OpAdd(_) => match &*pin.remotes {
                [] => {
                    ui.label("None");
                    PinInfo::square().with_fill(Color32::RED)
                }
                [remote] => {
                    let new_value = match snarl[remote.node].clone() {
                        Node::Float(value) => value,
                        Node::OpAdd(value) => value.value(),
                        _ => unimplemented!(),
                    };
                    if let Node::OpAdd(value) = &mut snarl[pin.id.node] {
                        if pin.id.input == 0 {
                            value.a = new_value;
                        } else {
                            value.b = new_value;
                        }
                    }
                    ui.label(format!("{}", new_value));
                    PinInfo::circle().with_fill(Color32::GREEN)
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> egui_snarl::ui::PinInfo {
        match &mut snarl[pin.id.node] {
            Node::Float(ref mut value) => {
                ui.add(egui::DragValue::new(value));
                if pin.remotes.len() > 0 {
                    PinInfo::circle().with_fill(Color32::GREEN)
                } else {
                    PinInfo::circle().with_fill(Color32::RED)
                }
            }
            Node::OpAdd(add_value) => {
                ui.label(format!("{}", add_value.value()));
                if pin.remotes.len() > 0 {
                    PinInfo::circle().with_fill(Color32::GREEN)
                } else {
                    PinInfo::circle().with_fill(Color32::RED)
                }
            }
            _ => unimplemented!(),
        }
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, _scale: f32, snarl: &mut Snarl<Node>) {
        ui.label("Add node");
        if ui.button("Float").clicked() {
            snarl.insert_node(pos, Node::Float(0.));
            ui.close_menu();
        }
        if ui.button("Output").clicked() {
            snarl.insert_node(pos, Node::Output());
            ui.close_menu();
        }
        ui.menu_button("Operations", |ui| {
            if ui.button("Add").clicked() {
                snarl.insert_node(pos, Node::OpAdd(AddNode::default()));
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
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}
