use domaker_parser::parser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] parser::ParseErrors),
    // #[error("Codegen error: {0}")]
    // Codegen(#[from] codegen::Error),
    // #[error("Transpiler error: {0}")]
    // Transpiler(#[from] transpiler::Error),
}
