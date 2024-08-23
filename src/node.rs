use egui_snarl::ui::PinInfo;
use glam::{Vec2, Vec3};

pub mod fragment;
pub mod math;

pub trait NodeView<T> {
    fn title(&self) -> String;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn connect(&self, index: usize, other: &T, other_index: usize) -> bool;
    fn has_body(&self) -> bool;
    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<T>);
    fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<(usize, T)>)
        -> PinInfo;
    fn show_output(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        remotes: &Vec<(usize, T)>,
    ) -> PinInfo;
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<T>;
}

#[derive(Clone, Copy, Debug)]
pub enum NodeValue {
    F32(f32),
    Vec2(Vec2),
    Vec3(Vec3),
}

impl NodeValue {
    fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }

    pub fn to_string(&self) -> String {
        match self {
            NodeValue::F32(value) => Self::format_float(*value),
            NodeValue::Vec2(value) => {
                format!(
                    "({}, {})",
                    Self::format_float(value.x),
                    Self::format_float(value.y)
                )
            }
            NodeValue::Vec3(value) => {
                format!(
                    "({}, {}, {})",
                    Self::format_float(value.x),
                    Self::format_float(value.y),
                    Self::format_float(value.z)
                )
            }
        }
    }

    pub fn value_label(&self, index: usize) -> String {
        match self {
            NodeValue::F32(_) => "x".to_string(),
            NodeValue::Vec2(_) => {
                if index == 0 {
                    "x".to_string()
                } else if index == 1 {
                    "y".to_string()
                } else if index == 2 {
                    "xy".to_string()
                } else {
                    unimplemented!()
                }
            }
            NodeValue::Vec3(_) => {
                if index == 0 {
                    "x".to_string()
                } else if index == 1 {
                    "y".to_string()
                } else if index == 2 {
                    "z".to_string()
                } else if index == 3 {
                    "xyz".to_string()
                } else {
                    unimplemented!()
                }
            }
        }
    }
}
