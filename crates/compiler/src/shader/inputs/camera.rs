use node_model::{CameraPositionNode, NodeData};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const CAM_PREFIX: &str = "cam";
const CAM_VAR_NAME: &str = "{prefix}_{id}.{label}";
const CAM_TEMPLATE: &str = "vec3 {prefix}_{id} = vec3({x}, {y}, {z});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for CameraPositionNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(CAM_VAR_NAME)
                .param("prefix", CAM_PREFIX)
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
        vec![CodeFragment {
            code: Template::builder(CAM_TEMPLATE)
                .param("prefix", CAM_PREFIX)
                .param("id", id)
                .float("x", self.x)
                .float("y", self.y)
                .float("z", self.z)
                .build(),
            section: ShaderSection::Main,
        }]
    }
}
