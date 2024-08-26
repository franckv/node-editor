mod math;
mod simple;

pub use math::MathNode;
pub use simple::SimpleNode;

pub trait GraphView<T> {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<T>;
}
