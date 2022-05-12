use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Real(f64),
    String(String),
    Bool(bool),
    Array(Box<[Object]>),
}

impl Object {
    pub fn to_c_lit(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::String(s) => s.clone(),
            Object::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Object::Array(a) => {
                let mut to_out = "{".to_string();
                for el in a.iter() {
                    to_out = format!("{}{},", to_out, el.to_c_lit());
                }
                to_out.pop();
                to_out += "}";
                to_out
            }
            Object::Real(r) => r.to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Object::Int(v) => v.to_string(),
            Object::String(v) => v.to_string(),
            Object::Bool(v) => v.to_string(),
            Object::Array(_a) => String::from_str("string").unwrap(),
            Object::Real(v) => v.to_string(),
        };
        let typ = match self {
            Object::Int(_) => String::from_str("int"),
            Object::String(_) => String::from_str("string"),
            Object::Bool(_) => String::from_str("bool"),
            Object::Array(_) => String::from_str("array"),
            Object::Real(_) => String::from_str("real"),
        }
        .unwrap();
        write!(f, "(v: {}, t: {})", val, typ)
    }
}
