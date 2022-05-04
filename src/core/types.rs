use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SimpleType {
    Int,
    String,
    Bool,
    Void,
}

impl Display for SimpleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SimpleType::Int => "int",
                SimpleType::String => "string",
                SimpleType::Bool => "bool",
                SimpleType::Void => "void",
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Type {
    Simple(SimpleType),
    Array(SimpleType),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Simple(t) => write!(f, "{}", t),
            Type::Array(a) => write!(f, "array of {}", a),
        }
    }
}

impl Type {
    pub fn internal(&self) -> SimpleType {
        match self {
            Type::Simple(s) => s.clone(),
            Type::Array(s) => s.clone(),
        }
    }
}
