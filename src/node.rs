#[derive(Clone, Debug)]
pub enum Node {
    Output(),
    Float(f32),
    OpAdd(AddNode),
}

#[derive(Clone, Default, Debug)]
pub struct AddNode {
    pub a: f32,
    pub b: f32,
}

impl AddNode {
    pub fn value(&self) -> f32 {
        self.a + self.b
    }
}
