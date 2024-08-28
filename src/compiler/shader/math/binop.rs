use crate::{
    compiler::{CodeFragment, NodeCompile, NodeParam, ShaderCompiler, ShaderSection},
    node::{math::Ops, BinOpNode, NodeView},
    template::Template,
};

const BINOP_VAR_NAME: &str = "{op_name}_{id}";
const BINOP_TEMPLATE: &str = "float {op_name}_{id} = {a} {op_sym} {b};";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for BinOpNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(BINOP_VAR_NAME)
                .param_dbg("op_name", self.op)
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
        let input_a = input_vars.get(0).expect("2 args");
        let input_b = input_vars.get(1).expect("2 args");

        let op = match self.op {
            Ops::Add => "+",
            Ops::Sub => "-",
            Ops::Mul => "*",
            Ops::Div => "/",
        };

        let code = if let (Some(a), Some(b)) = (input_a, input_b) {
            Template::builder(BINOP_TEMPLATE)
                .param_dbg("op_name", self.op)
                .param("id", id)
                .param("a", &a.name)
                .param("b", &b.name)
                .param("op_sym", op)
                .build()
        } else {
            "".to_string()
        };

        vec![CodeFragment {
            code: code,
            section: ShaderSection::Code,
        }]
    }
}
