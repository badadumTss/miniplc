use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Boolean,
    String,
    Integer,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub actual_type: Type,
    pub int_value: i64,
    pub string_value: String,
    pub bool_value: bool,
}

impl Variable {
    pub fn from_int(int_value_: i64) -> Variable {
        Variable {
            actual_type: Type::Integer,
            int_value: int_value_,
            string_value: int_value_.to_string(),
            bool_value: int_value_ != 0,
        }
    }

    pub fn from_string(string_value: String) -> Variable {
        Variable {
            actual_type: Type::String,
            int_value: match string_value.parse::<i64>() {
                Ok(val) => val,
                Err(_) => {
                    if string_value.is_empty() {
                        0
                    } else {
                        1
                    }
                }
            },
            string_value: string_value.clone(),
            bool_value: !string_value.clone().is_empty(),
        }
    }

    pub fn from_bool(bool_value_: bool) -> Variable {
        Variable {
            actual_type: Type::Boolean,
            int_value: if bool_value_ { 1 } else { 0 },
            string_value: if bool_value_ {
                "true".to_string()
            } else {
                "false".to_string()
            },
            bool_value: bool_value_,
        }
    }

    pub fn make_int(&mut self) {
        self.actual_type = Type::Integer;
    }

    pub fn make_string(&mut self) {
        self.actual_type = Type::String;
    }

    pub fn make_bool(&mut self) {
        self.actual_type = Type::Boolean;
    }
}
