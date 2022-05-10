use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SimpleType {
    Int,
    String,
    Bool,
    Void,
}

impl SimpleType {
    pub fn to_c_type(&self) -> String {
        match self {
            SimpleType::Int => "int".to_string(),
            SimpleType::String => "*char".to_string(),
            SimpleType::Bool => "bool".to_string(),
            SimpleType::Void => "void".to_string(),
        }
    }
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
            Type::Simple(s) => *s,
            Type::Array(s) => *s,
        }
    }

    pub fn to_c_type(&self) -> String {
        match self {
            Type::Simple(s) => s.to_c_type(),
            Type::Array(a) => format!("*{}", a.to_c_type()),
        }
    }
}
