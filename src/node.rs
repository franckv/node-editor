use egui_snarl::ui::PinInfo;

pub mod math;

pub trait NodeView<T> {
    fn title(&self) -> String;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn connect(&self, other: &T) -> bool;
    fn has_body(&self) -> bool;
    fn show_body(&mut self, ui: &mut egui::Ui, inputs: &Vec<T>);
    fn show_input(&mut self, ui: &mut egui::Ui, index: usize, remotes: &Vec<T>) -> PinInfo;
    fn show_output(&mut self, ui: &mut egui::Ui, remotes: &Vec<T>) -> PinInfo;
    fn show_graph_menu(ui: &mut egui::Ui) -> Option<T>;
}
