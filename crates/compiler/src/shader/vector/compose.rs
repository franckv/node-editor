use node_model::{ComposeNode, Dim, NodeData};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const COMP_VAR_NAME: &str = "{type}_{id}";
const COMP_TEMPLATE2: &str = "vec2 {type}_{id} = vec2({x}, {y});";
const COMP_TEMPLATE3: &str = "vec3 {type}_{id} = vec3({x}, {y}, {z});";
const COMP_TEMPLATE4: &str = "vec4 {type}_{id} = vec4({x}, {y}, {z}, {w});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for ComposeNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(COMP_VAR_NAME)
                .param("type", "comp")
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
        let input_x = input_vars.get(0).expect("2 args");
        let input_y = input_vars.get(1).expect("2 args");

        if let (Some(x), Some(y)) = (input_x, input_y) {
            match self.dim {
                Dim::Vec2 => vec![CodeFragment {
                    code: Template::builder(COMP_TEMPLATE2)
                        .param("type", "comp")
                        .param("id", id)
                        .param("x", &x.name)
                        .param("y", &y.name)
                        .build(),
                    section: ShaderSection::Main,
                }],
                Dim::Vec3 => {
                    let input_z = input_vars.get(2).expect("3 args");
                    if let Some(z) = input_z {
                        vec![CodeFragment {
                            code: Template::builder(COMP_TEMPLATE3)
                                .param("type", "comp")
                                .param("id", id)
                                .param("x", &x.name)
                                .param("y", &y.name)
                                .param("z", &z.name)
                                .build(),
                            section: ShaderSection::Main,
                        }]
                    } else {
                        vec![]
                    }
                }
                Dim::Vec4 => {
                    let input_z = input_vars.get(2).expect("4 args");
                    let input_w = input_vars.get(3).expect("4 args");
                    if let (Some(z), Some(w)) = (input_z, input_w) {
                        vec![CodeFragment {
                            code: Template::builder(COMP_TEMPLATE4)
                                .param("type", "comp")
                                .param("id", id)
                                .param("x", &x.name)
                                .param("y", &y.name)
                                .param("z", &z.name)
                                .param("w", &w.name)
                                .build(),
                            section: ShaderSection::Main,
                        }]
                    } else {
                        vec![]
                    }
                }
            }
        } else {
            vec![]
        }
    }
}
