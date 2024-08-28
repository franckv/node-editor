use node_model::{FloatNode, NodeData};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const FLOAT_PREFIX: &str = "float";
const FLOAT_VAR_NAME: &str = "{prefix}_{id}";
const FLOAT_TEMPLATE: &str = "float {prefix}_{id} = {x};";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for FloatNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(FLOAT_VAR_NAME)
                .param("prefix", FLOAT_PREFIX)
                .param("id", id)
                .build(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(
        &self,
        id: usize,
        _input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        vec![CodeFragment {
            code: Template::builder(FLOAT_TEMPLATE)
                .param("prefix", FLOAT_PREFIX)
                .param("id", id)
                .float("x", self.value)
                .build(),
            section: ShaderSection::Main,
        }]
    }
}
