mod fragment;
mod math;

pub use fragment::FragmentNode;
pub use math::MathNode;

pub trait GraphView<T> {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<T>;
}
