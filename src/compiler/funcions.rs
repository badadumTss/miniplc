use log::trace;

use crate::core::{
    ast::{FunctionCallNode, FunctionDeclNode},
    types::{SimpleType, Type},
};

use super::Compiler;

impl Compiler {
    pub fn f_ret_value(name: String) -> String {
        format!("{}_return_value", name)
    }

    pub fn f_ret_ptr(name: String) -> String {
        format!("{}_return_ptr", name)
    }

    pub fn compile_function(&mut self, f: FunctionDeclNode) {
        self.scope = f.name.clone();
        trace!("Compiling function declaration");
        for sym in f.args.iter() {
            self.push_instruction(format!(
                "{} {}_{};",
                sym.r_type.to_c_type(),
                f.name,
                sym.name
            ));
        }
        self.push_instruction(format!(
            "{} {};",
            f.r_type.to_c_type(),
            Compiler::f_ret_value(f.name.clone())
        ));
        self.push_instruction(format!("void* {};", Compiler::f_ret_ptr(f.name.clone())));
        self.push_instruction(format!("void* fptr_{} = &&f_{};", f.name, f.name));
        self.push_label(format!("f_{}", f.name));
        self.compile_ast(*f.block);
        self.push_instruction(format!("goto *{};", Compiler::f_ret_ptr(f.name)))
    }

    pub fn type_for_last(r_type: Type) -> &'static str {
        match r_type {
            Type::Simple(a) => match a {
                SimpleType::Int => "int",
                SimpleType::String => "str",
                SimpleType::Bool => "bool",
                SimpleType::Void => "void",
            },
            Type::Array(a) => match a {
                SimpleType::Int => "int_arr",
                SimpleType::String => "str_arr",
                SimpleType::Bool => "bool_arr",
                SimpleType::Void => "void_arr",
            },
        }
    }

    pub fn compile_function_call(&mut self, f: FunctionCallNode) {
        trace!("Compiling function call");
        let label = self.advance_label();
        for arg in f.args.iter() {
            self.compile_ast(arg.1.clone());
            let r_type = Compiler::type_for_last(arg.1.r_type());
            self.push_instruction(format!("{}_{} = last_{};", f.target, arg.0, r_type));
        }
        self.push_instruction(format!(
            "{} = &&ret_{};",
            Compiler::f_ret_ptr(f.clone().target),
            label
        ));
        self.push_instruction(format!("goto f_{};", f.target));
        self.push_label(format!("ret_{}", label));
        self.push_instruction(format!(
            "last_{} = {};",
            Compiler::type_for_last(f.r_type),
            Compiler::f_ret_value(f.target)
        ));
    }
}
