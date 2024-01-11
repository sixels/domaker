use super::Property;

#[derive(Debug)]
pub struct Type<'input> {
    pub kind: TypeKind<'input>,
    pub optional: bool,
    pub properties: Vec<Property<'input>>,
    pub generics: Vec<Type<'input>>,
}

#[derive(Debug, Clone)]
pub enum TypeKind<'input> {
    Int(IntType),
    Float(FloatType),
    Boolean,
    String,
    Timestamp,
    Uuid,
    Json,
    List,
    Schema(&'input str),
}

#[derive(Debug, Clone, Copy)]
pub struct IntType {
    pub width: IntWidth,
    pub signed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum IntWidth {
    W8,
    W16,
    W32,
    W64,
}

#[derive(Debug, Clone, Copy)]
pub struct FloatType {
    pub width: FloatWidth,
}

#[derive(Debug, Clone, Copy)]
pub enum FloatWidth {
    W32,
    W64,
}

impl<'input> From<&'input str> for TypeKind<'input> {
    fn from(value: &'input str) -> Self {
        match value {
            "Int8" => Self::Int(IntType {
                width: IntWidth::W8,
                signed: true,
            }),
            "Int16" => Self::Int(IntType {
                width: IntWidth::W16,
                signed: true,
            }),
            "Int32" => Self::Int(IntType {
                width: IntWidth::W32,
                signed: true,
            }),
            "Int64" => Self::Int(IntType {
                width: IntWidth::W64,
                signed: true,
            }),
            "Uint8" => Self::Int(IntType {
                width: IntWidth::W8,
                signed: false,
            }),
            "Uint16" => Self::Int(IntType {
                width: IntWidth::W16,
                signed: false,
            }),
            "Uint32" => Self::Int(IntType {
                width: IntWidth::W32,
                signed: false,
            }),
            "Uint64" => Self::Int(IntType {
                width: IntWidth::W64,
                signed: false,
            }),
            "Float32" => Self::Float(FloatType {
                width: FloatWidth::W32,
            }),
            "Float64" => Self::Float(FloatType {
                width: FloatWidth::W64,
            }),
            "Boolean" => Self::Boolean,
            "String" => Self::String,
            "Timestamp" => Self::Timestamp,
            "Uuid" => Self::Uuid,
            "Json" => Self::Json,
            _ => Self::Schema(value),
        }
    }
}
