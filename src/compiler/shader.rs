mod constants;
mod display;
mod fragment;
mod math;
mod vector;

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
