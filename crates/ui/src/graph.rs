mod fragment;
mod simple;

pub use fragment::FragmentShaderNode;
pub use simple::SimpleNode;

pub trait GraphView<T> {
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<T>;
}
