mod constants;
mod inputs;
mod math;
mod outputs;
mod vector;

use crate::{compiler::ScriptGenerator, utils::Template, CodeFragment};

const FRAGMENT_SHADER: &str = r"
    #version 450

    {declarations}

    void main() {
	    {main}
    }
";

pub struct ShaderCompiler;

impl ScriptGenerator<ShaderSection> for ShaderCompiler {
    fn script(fragments: &Vec<CodeFragment<ShaderSection>>) -> String {
        let mut extensions = vec![];
        let mut includes = vec![];
        let mut declarations = vec![];
        let mut functions = vec![];
        let mut main = vec![];

        for fragment in fragments {
            match fragment.section {
                ShaderSection::Extensions => extensions.push(fragment.code.as_str()),
                ShaderSection::Includes => includes.push(fragment.code.as_str()),
                ShaderSection::Declaration => declarations.push(fragment.code.as_str()),
                ShaderSection::Function => functions.push(fragment.code.as_str()),
                ShaderSection::Main => main.push(fragment.code.as_str()),
            }
        }

        Template::builder(FRAGMENT_SHADER)
            .param("declarations", declarations.join("\n"))
            .param("main", main.join("\n"))
            .build()
    }
}

#[derive(Debug)]
pub enum ShaderSection {
    Extensions,
    Includes,
    Declaration,
    Function,
    Main,
}
