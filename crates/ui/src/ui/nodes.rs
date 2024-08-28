use node_model::{
    math::Ops, vector::DecomposeNode, BinOpNode, CameraPositionNode, ComposeNode, Dim, FloatNode,
    OutputNode, VecNode, VertexInNode,
};

use crate::view::NodeView;

impl<T> NodeView<T> for FloatNode<T> {
    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!();
    }
}

impl<T> NodeView<T> for VecNode<T> {
    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        let old_dim = self.dim;

        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.dim))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.dim, Dim::Vec2, "Vec2");
                ui.selectable_value(&mut self.dim, Dim::Vec3, "Vec3");
                ui.selectable_value(&mut self.dim, Dim::Vec4, "Vec4");
            });

        self.dim != old_dim
    }
}

impl<T> NodeView<T> for CameraPositionNode<T> {
    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!()
    }
}

impl<T> NodeView<T> for VertexInNode<T> {
    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!()
    }
}

impl<T> NodeView<T> for BinOpNode<T> {
    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        let old_op = self.op;

        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.op))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.op, Ops::Add, "Add");
                ui.selectable_value(&mut self.op, Ops::Sub, "Sub");
                ui.selectable_value(&mut self.op, Ops::Mul, "Mul");
                ui.selectable_value(&mut self.op, Ops::Div, "Div");
            });

        self.op != old_op
    }
}

impl<T> NodeView<T> for OutputNode<T> {
    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!();
    }
}

impl<T> NodeView<T> for ComposeNode<T> {
    fn has_body(&self) -> bool {
        true
    }

    fn show_body(&mut self, ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        let old_dim = self.dim;

        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.dim))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.dim, Dim::Vec2, "Vec2");
                ui.selectable_value(&mut self.dim, Dim::Vec3, "Vec3");
                ui.selectable_value(&mut self.dim, Dim::Vec4, "Vec4");
            });

        self.dim != old_dim
    }
}

impl<T> NodeView<T> for DecomposeNode<T> {
    fn has_body(&self) -> bool {
        false
    }

    fn show_body(&mut self, _ui: &mut egui::Ui, _inputs: &Vec<T>) -> bool {
        unimplemented!();
    }
}
