use crate::compiler::{NodeCompile, NodeParam};
use crate::node::vector::Dim;
use crate::node::{
    math::Ops, BinOpNode, CameraPositionNode, ComposeNode, FloatNode, NodeView, OutputNode,
    Vec2Node,
};

pub struct ShaderCompiler;

impl<T> NodeCompile<T, ShaderCompiler> for FloatNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("float_{}", id),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(&self, id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        let var = &self.out_vars(id, 0);

        format!("float {} = {:.2};", &var.name, self.value)
    }
}

const VEC_VAR_NAME: &str = "vec_";

impl<T> NodeCompile<T, ShaderCompiler> for Vec2Node<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{}{}.{}", VEC_VAR_NAME, id, self.outputs()[index].label),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(&self, id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        let var = format!("{}{}", VEC_VAR_NAME, id);

        format!("vec2 {} = vec2({:.2}, {:.2});", var, self.x, self.y)
    }
}

impl<T> NodeCompile<T, ShaderCompiler> for OutputNode<T> {
    fn out_vars(&self, _id: usize, _index: usize) -> NodeParam {
        unimplemented!()
    }

    fn code(&self, _id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        "".to_string()
    }
}

const CAM_VAR_NAME: &str = "camera_";

impl<T> NodeCompile<T, ShaderCompiler> for CameraPositionNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{}{}.{}", CAM_VAR_NAME, id, self.outputs()[index].label),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(&self, id: usize, _input_vars: &Vec<Option<NodeParam>>) -> String {
        let var = format!("{}{}", CAM_VAR_NAME, id);

        format!(
            "vec3 {} = vec3({:.2}, {:.2}, {:.2});",
            var, self.x, self.y, self.z
        )
    }
}

impl<T> NodeCompile<T, ShaderCompiler> for BinOpNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{:?}_{}", self.op, id).to_lowercase(),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(&self, id: usize, input_vars: &Vec<Option<NodeParam>>) -> String {
        let input_a = input_vars.get(0).expect("2 args");
        let input_b = input_vars.get(1).expect("2 args");

        let var = &self.out_vars(id, 0);

        let op = match self.op {
            Ops::Add => "+",
            Ops::Sub => "-",
            Ops::Mul => "*",
            Ops::Div => "/",
        };

        let code = if let (Some(a), Some(b)) = (input_a, input_b) {
            format!("float {} = {} {} {};", &var.name, &a.name, op, &b.name)
        } else {
            "".to_string()
        };

        code
    }
}

const COMP_VAR_NAME: &str = "compose_";

impl<T> NodeCompile<T, ShaderCompiler> for ComposeNode<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam {
        NodeParam {
            name: format!("{}{}", COMP_VAR_NAME, id),
            ty: self.outputs()[index].ty,
        }
    }

    fn code(&self, id: usize, input_vars: &Vec<Option<NodeParam>>) -> String {
        let input_x = input_vars.get(0).expect("2 args");
        let input_y = input_vars.get(1).expect("2 args");

        let var = format!("{}{}", COMP_VAR_NAME, id);

        let code = if let (Some(x), Some(y)) = (input_x, input_y) {
            match self.dim {
                Dim::Vec2 => {
                    format!("vec2 {} = vec2({}, {});", var, &x.name, &y.name)
                }
                Dim::Vec3 => {
                    let input_z = input_vars.get(2).expect("3 args");
                    if let Some(z) = input_z {
                        format!(
                            "vec3 {} = vec3({}, {}, {});",
                            var, &x.name, &y.name, &z.name
                        )
                    } else {
                        String::from("")
                    }
                }
                Dim::Vec4 => {
                    let input_z = input_vars.get(2).expect("4 args");
                    let input_w = input_vars.get(3).expect("4 args");
                    if let (Some(z), Some(w)) = (input_z, input_w) {
                        format!(
                            "vec4 {} = vec4({}, {}, {}, {});",
                            var, &x.name, &y.name, &z.name, &w.name
                        )
                    } else {
                        String::from("")
                    }
                }
            }
        } else {
            String::from("")
        };

        code
    }
}
