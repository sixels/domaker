use super::{Literal, Type};

// TODO: implement generics for schema
#[derive(Debug)]
pub enum Schema<'input> {
    Definition(SchemaDefinition<'input>),
    Alias(SchemaAlias<'input>),
}

#[derive(Debug)]
pub struct SchemaDefinition<'input> {
    pub name: &'input str,
    pub fields: Vec<Field<'input>>,
    pub properties: Vec<&'input str>,
}

#[derive(Debug)]
pub struct SchemaAlias<'input> {
    pub name: &'input str,
    pub ty: Type<'input>,
    pub properties: Vec<&'input str>,
}

#[derive(Debug)]
pub struct Field<'input> {
    pub name: &'input str,
    pub ty: Type<'input>,
    pub doc_comment: Vec<&'input str>,
}

#[derive(Debug)]
pub struct Property<'input> {
    pub name: &'input str,
    pub value: PropertyValue<'input>,
}

#[derive(Debug)]
pub enum PropertyValue<'input> {
    Ident(&'input str),
    Literal(Literal<'input>),
}
