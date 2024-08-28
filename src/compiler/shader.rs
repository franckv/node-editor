use crate::compiler::{CodeFragment, NodeCompile, NodeParam};
use crate::node::{
    math::Ops, BinOpNode, CameraPositionNode, ComposeNode, Dim, FloatNode, NodeView, OutputNode,
    VecNode,
};
use crate::template::Template;

pub struct ShaderCompiler;

#[derive(Debug)]
pub enum ShaderSection {
    Extensions,
    Includes,
    Declaration,
    Function,
    Code,
    Noop,
}

const FLOAT_VAR_NAME: &str = "{type}_{id}";
const FLOAT_TEMPLATE: &str = "float {type}_{id} = {x};";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for FloatNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(FLOAT_VAR_NAME)
                .param("type", "float")
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
                .param("type", "float")
                .param("id", id)
                .float("x", self.value)
                .build(),
            section: ShaderSection::Code,
        }]
    }
}

const VEC_VAR_NAME: &str = "{type}_{id}.{label}";
const VEC_TEMPLATE2: &str = "vec2 {type}_{id} = vec2({x}, {y});";
const VEC_TEMPLATE3: &str = "vec3 {type}_{id} = vec3({x}, {y}, {z});";
const VEC_TEMPLATE4: &str = "vec4 {type}_{id} = vec4({x}, {y}, {z}, {w});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for VecNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(VEC_VAR_NAME)
                .param("type", "vec")
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
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .build(),
                section: ShaderSection::Code,
            }],
            Dim::Vec3 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE3)
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .build(),
                section: ShaderSection::Code,
            }],
            Dim::Vec4 => vec![CodeFragment {
                code: Template::builder(VEC_TEMPLATE4)
                    .param("type", "vec")
                    .param("id", id)
                    .float("x", self.x)
                    .float("y", self.y)
                    .float("z", self.z)
                    .float("w", self.w)
                    .build(),
                section: ShaderSection::Code,
            }],
        }
    }
}

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

const CAM_VAR_NAME: &str = "{type}_{id}.{label}";
const CAM_TEMPLATE: &str = "vec3 {type}_{id} = vec3({x}, {y}, {z});";

impl<T> NodeCompile<T, ShaderCompiler, ShaderSection> for CameraPositionNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: Template::builder(CAM_VAR_NAME)
                .param("type", "cam")
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
                .param("type", "cam")
                .param("id", id)
                .float("x", self.x)
                .float("y", self.y)
                .float("z", self.z)
                .build(),
            section: ShaderSection::Code,
        }]
    }
}

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
                    section: ShaderSection::Code,
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
                            section: ShaderSection::Code,
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
                            section: ShaderSection::Code,
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
