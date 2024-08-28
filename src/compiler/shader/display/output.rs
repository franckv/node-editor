use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    node::OutputNode,
    template::Template,
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
            section: ShaderSection::Code,
        };

        let mut code = vec![decl];

        if let Some(input) = input {
            let template = match input.ty {
                crate::node::NodeValueType::F32 => unimplemented!(),
                crate::node::NodeValueType::Vec2 => OUTPUT_TEMPLATE_CODE2,
                crate::node::NodeValueType::Vec3 => OUTPUT_TEMPLATE_CODE3,
                crate::node::NodeValueType::Vec4 => OUTPUT_TEMPLATE_CODE4,
                crate::node::NodeValueType::Any => unimplemented!(),
                crate::node::NodeValueType::None => unimplemented!(),
            };
            code.push(CodeFragment {
                code: Template::builder(template)
                    .param("var", &input.name)
                    .build(),
                section: ShaderSection::Code,
            })
        }

        code
    }
}
