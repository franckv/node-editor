use glam::{Vec2, Vec3, Vec4};

pub mod constants;
pub mod display;
pub mod fragment;
pub mod math;
pub mod vector;

pub use constants::{FloatNode, Vec2Node};
pub use display::OutputNode;
pub use fragment::CameraPositionNode;
pub use math::BinOpNode;
pub use vector::ComposeNode;

pub trait NodeView<T> {
    fn out_value(&self, index: usize) -> NodeValue;
    fn f32_out_value_mut(&mut self, index: usize) -> &mut f32;
    fn in_value(&mut self, index: usize, value: NodeValue);
    fn title(&self) -> String;
    fn inputs(&self) -> &[Connector];
    fn outputs(&self) -> &[Connector];
    fn has_body(&self) -> bool;
    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<T>) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeValueType {
    F32,
    Vec2,
    Vec3,
    Vec4,
    Any,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum NodeValue {
    F32(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
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
            NodeValue::Vec4(value) => {
                format!(
                    "({}, {}, {}, {})",
                    Self::format_float(value.x),
                    Self::format_float(value.y),
                    Self::format_float(value.z),
                    Self::format_float(value.w)
                )
            }
        }
    }
}

pub struct Connector {
    pub ty: NodeValueType,
    pub label: &'static str,
    pub editable: bool,
}
