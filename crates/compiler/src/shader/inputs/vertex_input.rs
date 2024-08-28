use node_model::{NodeData, VertexInNode};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const VERT_IN_PREFIX: &str = "vertex_in";
const VERT_IN_VAR_NAME: &str = "{prefix}.{field}";
const VERT_IN_TEMPLATE: &str = r"layout(location = 0) in struct VertexInput {
	vec4 color;
} vertex_in;";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for VertexInNode<T> {
    fn out_vars(&self, _id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(VERT_IN_VAR_NAME)
                .param("prefix", VERT_IN_PREFIX)
                .param("field", self.outputs()[index].label)
                .build(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(
        &self,
        _id: usize,
        _input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        vec![CodeFragment {
            code: VERT_IN_TEMPLATE.to_string(),
            section: ShaderSection::Declaration,
        }]
    }
}
