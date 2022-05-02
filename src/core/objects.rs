use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    String(String),
    Bool(bool),
    Array(Box<[Object]>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Object::Int(v) => v.to_string(),
            Object::String(v) => v.to_string(),
            Object::Bool(v) => v.to_string(),
            Object::Array(a) => String::from_str("string").unwrap(),
        };
        let typ = match self {
            Object::Int(_) => String::from_str("int"),
            Object::String(_) => String::from_str("string"),
            Object::Bool(_) => String::from_str("bool"),
            Object::Array(_) => String::from_str("array"),
        }
        .unwrap();
        write!(f, "(v: {}, t: {})", val, typ)
    }
}
