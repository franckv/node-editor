use egui_snarl::ui::PinInfo;

use crate::node::{math::Node, NodeValue, NodeView};

#[derive(Clone, Default, Debug, serde::Serialize)]
pub struct ComposeNode {
    x: f32,
    y: f32,
}

impl ComposeNode {
    pub fn values(&self) -> NodeValue {
        NodeValue::Vec2((self.x, self.y).into())
    }

    pub fn value(&self, index: usize) -> NodeValue {
        if index == 0 {
            NodeValue::F32(self.x)
        } else if index == 1 {
            NodeValue::F32(self.y)
        } else {
            NodeValue::Vec2((self.x, self.y).into())
        }
    }

    pub fn value_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }

    pub fn set_value(&mut self, index: usize, value: NodeValue) {
        match value {
            NodeValue::F32(value) => {
                if index == 0 {
                    self.x = value;
                } else {
                    self.y = value;
                }
            }
            NodeValue::Vec2(value) => {
                if index == 0 {
                    self.x = value.x;
                } else {
                    self.y = value.y;
                }
            }
            NodeValue::Vec3(value) => {
                if index == 0 {
                    self.x = value.x;
                } else {
                    self.y = value.y;
                }
            }
        }
    }
}

impl NodeView<Node> for ComposeNode {
    fn title(&self) -> String {
        "Compose".to_string()
    }

    fn inputs(&self) -> usize {
        2
    }

    fn outputs(&self) -> usize {
        1
    }

    fn connect(&self, _: usize, other: &Node, _: usize) -> bool {
        match other {
            Node::Output(_) => true,
            _ => false,
        }
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
        index: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        if remotes.len() == 0 {
            ui.label("None");
            Node::get_pin_float_disconnected()
        } else {
            let (r_index, remote_node) = &remotes[0];
            let new_value = match remote_node {
                Node::Float(value) => value.value(),
                Node::BinOp(value) => value.value(),
                Node::Vec2(value) => value.value(*r_index),
                _ => unimplemented!(),
            };
            self.set_value(index, new_value);

            ui.label(new_value.to_string());
            Node::get_pin_float_connected()
        }
    }

    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        _: usize,
        remotes: &Vec<(usize, Node)>,
    ) -> PinInfo {
        ui.label(self.values().to_string());

        if remotes.len() > 0 {
            Node::get_pin_vec_connected()
        } else {
            Node::get_pin_vec_disconnected()
        }
    }

    fn show_graph_menu(_: &mut egui::Ui) -> Option<Node> {
        unimplemented!();
    }
}
