use node_model::{math::OneMinusNode, NodeData};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const ONEMINUS_PREFIX: &str = "oneminus";
const ONEMINUS_VAR_NAME: &str = "{prefix}_{id}";
const ONEMINUS_TEMPLATE: &str = "float {prefix}_{id} = 1.0 - {x};";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for OneMinusNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(ONEMINUS_VAR_NAME)
                .param("prefix", ONEMINUS_PREFIX)
                .param("id", id)
                .build(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(
        &self,
        id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        let input_x = input_vars.get(0).expect("1 arg");

        if let Some(x) = input_x {
            vec![CodeFragment {
                code: Template::builder(ONEMINUS_TEMPLATE)
                    .param("prefix", ONEMINUS_PREFIX)
                    .param("id", id)
                    .param("x", &x.name)
                    .build(),
                section: ShaderSection::Main,
            }]
        } else {
            vec![]
        }
    }
}
