pub mod schema;
pub mod types;

pub use schema::*;
pub use types::*;

#[derive(Debug)]
pub struct Module<'input> {
    pub name: &'input str,
    pub statements: Vec<Statement<'input>>,
}

#[derive(Debug)]
pub enum Statement<'input> {
    Schema(Schema<'input>),
    Port(Port<'input>),
}

#[derive(Debug)]
pub struct Port<'input> {
    pub name: &'input str,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Schema,
    Port,
    Module,
}

#[derive(Debug)]
pub enum Literal<'input> {
    String(&'input str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
