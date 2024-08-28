use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    node::{Dim, NodeView, VecNode},
    template::Template,
};

const VEC_VAR_NAME: &str = "{type}_{id}.{label}";
const VEC_TEMPLATE2: &str = "vec2 {type}_{id} = vec2({x}, {y});";
const VEC_TEMPLATE3: &str = "vec3 {type}_{id} = vec3({x}, {y}, {z});";
const VEC_TEMPLATE4: &str = "vec4 {type}_{id} = vec4({x}, {y}, {z}, {w});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for VecNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(VEC_VAR_NAME)
                .param("type", "vec")
                .param("id", id)
                .param("label", self.outputs()[index].label)
                .build(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(
        &self,
        id: usize,
        _input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        match self.dim {
            Dim::Vec2 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE2)
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .build(),
                section: ShaderSection::Code,
            }],
            Dim::Vec3 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE3)
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .build(),
                section: ShaderSection::Code,
            }],
            Dim::Vec4 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE4)
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .float("w", self.w)
                    .build(),
                section: ShaderSection::Code,
            }],
        }
    }
}
