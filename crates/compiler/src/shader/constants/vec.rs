use node_model::{Dim, NodeData, VecNode};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const VEC_PREFIX: &str = "vec";
const VEC_VAR_NAME: &str = "{prefix}_{id}";
const VEC_VAR_NAME_LABEL: &str = "{prefix}_{id}.{label}";
const VEC_TEMPLATE2: &str = "vec2 {prefix}_{id} = vec2({x}, {y});";
const VEC_TEMPLATE3: &str = "vec3 {prefix}_{id} = vec3({x}, {y}, {z});";
const VEC_TEMPLATE4: &str = "vec4 {prefix}_{id} = vec4({x}, {y}, {z}, {w});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for VecNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        let template = if self.dim == Dim::Vec2 && index == 2 {
            VEC_VAR_NAME
        } else if self.dim == Dim::Vec3 && index == 3 {
            VEC_VAR_NAME
        } else if self.dim == Dim::Vec4 && index == 4 {
            VEC_VAR_NAME
        } else {
            VEC_VAR_NAME_LABEL
        };

        NodeParam {
            name: Template::builder(template)
                .param("prefix", VEC_PREFIX)
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
                    .param("prefix", VEC_PREFIX)
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .build(),
                section: ShaderSection::Main,
            }],
            Dim::Vec3 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE3)
                    .param("prefix", VEC_PREFIX)
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .build(),
                section: ShaderSection::Main,
            }],
            Dim::Vec4 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE4)
                    .param("prefix", VEC_PREFIX)
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .float("w", self.w)
                    .build(),
                section: ShaderSection::Main,
            }],
        }
    }
}
