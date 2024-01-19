use domaker_parser::ast::{FloatType, IntType};

pub struct IR {
    pub module_name: String,
    pub filename: String,
    pub schemas: Vec<SchemaIR>,
    pub ports: Vec<PortIR>,
}

pub struct SchemaIR {
    pub name: String,
    pub fields: Vec<FieldIR>,
}

pub struct FieldIR {
    pub name: String,
    pub ty: TypeIR,
}

pub enum TypeIR {
    Int(IntType),
    Float(FloatType),
    String,
    Boolean,
    List(Box<TypeIR>),
    Optional(Box<TypeIR>),
    Reference(String),
}

pub struct PortIR {
    pub name: String,
}
