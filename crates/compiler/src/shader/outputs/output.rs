use node_model::{NodeValueType, OutputNode};

use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    utils::Template,
};

const OUTPUT_TEMPLATE_DECL: &str = "layout(location = 0) out vec4 out_color;";
const OUTPUT_TEMPLATE_CODE2: &str = "out_color = vec4({var}, 0.0, 1.0);";
const OUTPUT_TEMPLATE_CODE3: &str = "out_color = vec4({var}, 1.0);";
const OUTPUT_TEMPLATE_CODE4: &str = "out_color = {var};";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for OutputNode<T> {
    fn out_vars(&self, _id: usize, _index: usize) -> NodeParam {
        unimplemented!()
    }

    fn code(
        &self,
        _id: usize,
        input_vars: &Vec<Option<NodeParam>>,
    ) -> Vec<CodeFragment<ShaderSection>> {
        let input = input_vars.get(0).expect("1 args");

        let decl = CodeFragment {
            code: OUTPUT_TEMPLATE_DECL.to_string(),
            section: ShaderSection::Declaration,
        };

        let mut code = vec![decl];

        if let Some(input) = input {
            let template = match input.ty {
                NodeValueType::F32 => unimplemented!(),
                NodeValueType::Vec2 => OUTPUT_TEMPLATE_CODE2,
                NodeValueType::Vec3 => OUTPUT_TEMPLATE_CODE3,
                NodeValueType::Vec4 => OUTPUT_TEMPLATE_CODE4,
                NodeValueType::Any => unimplemented!(),
                NodeValueType::None => unimplemented!(),
            };
            code.push(CodeFragment {
                code: Template::builder(template)
                    .param("var", &input.name)
                    .build(),
                section: ShaderSection::Main,
            })
        }

        code
    }
}
