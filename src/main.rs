use domaker_parser::{
    ast::{FloatType, IntType, Module, Schema, Statement, TypeKind},
    parser::Parser,
};

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

fn main() {
    let input_file = std::env::args().nth(1).expect("input file is required");
    let input = std::fs::read_to_string(input_file).expect("failed to read input file");

    let Ok(module_ast) = Parser::parse("foo.dmk", &input).unwrap() else {
        panic!("failed to parse input file");
    };

    // let schemas = Vec::new();
    // for statement in module_ast.statements {
    //     match statement {
    //         Statement::Schema(schema) => match schema {
    //             Schema::Definition(definition) => {
    //                 let fields = Vec::new();
    //                 for field in definition.fields {
    //                     // let ty = match field.ty
    //                     match field.ty.kind {
    //                         TypeKind::Int(IntType {signed})
    //                     }
    //                     // fields.push(FieldIR {
    //                     //     name: field.name.to_string(),
    //                     //     ty,
    //                     // });
    //                 }
    //             }
    //             Schema::Alias(alias) => {
    //                 todo!()
    //             }
    //         },
    //         Statement::Port(port) => {
    //             todo!()
    //         }
    // }
    // }
}
