use crate::core::{
    ast::PrintStmtNode,
    types::{SimpleType, Type},
};

use super::Compiler;

impl Compiler {
    pub fn compile_print(&mut self, node: PrintStmtNode) {
        let child = node.to_print.as_ref();
        self.compile_ast(child.clone());
        match child.r_type() {
            Type::Simple(t) => match t {
                SimpleType::Int => self.emit("printf(\"%d\\n\", last_int);".to_string()),
                SimpleType::String => self.emit("printf(\"%s\\n\", last_str);".to_string()),
                SimpleType::Bool => self.emit("printf(\"%d\\n\", last_bool);".to_string()),
                SimpleType::Real => self.emit("printf(\"%f\\n\", last_float);".to_string()),
                SimpleType::Void => {
                    self.push_c_error(child.clone(), "Unable to print a void expression");
                }
            },
            Type::Array(t) => self.emit(format!(
                "printf(\"array of {}\");",
                match t {
                    SimpleType::Int => "int",
                    SimpleType::Real => "reals",
                    SimpleType::String => "string",
                    SimpleType::Bool => "bool",
                    SimpleType::Void => "void",
                }
            )),
        }
    }
}
