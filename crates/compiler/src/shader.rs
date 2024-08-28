mod constants;
mod outputs;
mod inputs;
mod math;
mod vector;

pub struct ShaderCompiler;

#[derive(Debug)]
pub enum ShaderSection {
    Extensions,
    Includes,
    Declaration,
    Function,
    Main,
}
