pub mod error;

use std::{ffi::OsStr, fs, io, path::Path};

use domaker_parser::{ast::Module, parser::ModuleParser};

use crate::ir::IR;

use self::error::Error;

pub struct Transpiler {
    content: String,
    filename: String,
}

impl Transpiler {
    pub fn new<P: AsRef<Path>>(input_file: P) -> io::Result<Self> {
        let input = fs::read_to_string(&input_file)?;

        let filename: String = input_file
            .as_ref()
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_string();

        Ok(Self {
            content: input,
            filename,
        })
    }

    pub fn transpile(&self) -> Result<(), Error> {
        let c = &self.content;
        let module_ast: Module = ModuleParser::new().parse(c)?;
        // let ir = self.transpile_module(self.filename.clone(), &module_ast);

        Ok(())
    }

    fn generate_ir(&self, filename: String, module: &Module) -> IR {
        let schemas = Vec::new();
        let ports = Vec::new();

        IR {
            module_name: module.name.to_string(),
            filename: String::new(),
            schemas,
            ports,
        }
    }
}
