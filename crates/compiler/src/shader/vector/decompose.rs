use node_model::{vector::DecomposeNode, NodeData};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const DECOMPOSE_PREFIX: &str = "decomp";
const DECOMPOSE_VAR_NAME: &str = "{prefix}_{id}.{label}";
const DECOMPOSE_TEMPLATE2: &str = "vec2 {prefix}_{id} = {var}";
const DECOMPOSE_TEMPLATE3: &str = "vec3 {prefix}_{id} = {var}";
const DECOMPOSE_TEMPLATE4: &str = "vec4 {prefix}_{id} = {var}";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for DecomposeNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(DECOMPOSE_VAR_NAME)
                .param("prefix", DECOMPOSE_PREFIX)
                .param("id", id)
                .param("label", self.outputs()[index].label)
                .build(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(
        &self,
        id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        let input_var = input_vars.get(0).expect("1 arg");

        if let Some(var) = input_var {
            match self.dim {
                node_model::Dim::Vec2 => vec![CodeFragment {
                    code: Template::builder(DECOMPOSE_TEMPLATE2)
                        .param("prefix", DECOMPOSE_PREFIX)
                        .param("id", id)
                        .param("var", &var.name)
                        .build(),
                    section: ShaderSection::Main,
                }],
                node_model::Dim::Vec3 => vec![CodeFragment {
                    code: Template::builder(DECOMPOSE_TEMPLATE3)
                        .param("prefix", DECOMPOSE_PREFIX)
                        .param("id", id)
                        .param("var", &var.name)
                        .build(),
                    section: ShaderSection::Main,
                }],
                node_model::Dim::Vec4 => vec![CodeFragment {
                    code: Template::builder(DECOMPOSE_TEMPLATE4)
                        .param("prefix", DECOMPOSE_PREFIX)
                        .param("id", id)
                        .param("var", &var.name)
                        .build(),
                    section: ShaderSection::Main,
                }],
            }
        } else {
            vec![]
        }
    }
}
