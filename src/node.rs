use egui_snarl::ui::PinInfo;
use glam::{Vec2, Vec3};

pub mod fragment;
pub mod math;

pub trait NodeView<T> {
    fn out_value(&self, index: usize) -> NodeValue;
    fn in_value(&mut self, index: usize, value: NodeValue);
    fn title(&self) -> String;
    fn inputs(&self) -> &[(NodeValueType, &str)];
    fn outputs(&self) -> &[(NodeValueType, &str)];
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeValueType {
    F32,
    Vec2,
    Vec3,
    Any,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum NodeValue {
    F32(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    None,
}

impl NodeValue {
    fn format_float(value: f32) -> String {
        format!("{}", (value * 100.).round() / 100.)
    }

    pub fn to_string(&self) -> String {
        match self {
            NodeValue::None => "None".to_string(),
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
}
