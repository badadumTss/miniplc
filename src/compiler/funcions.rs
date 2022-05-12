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
            self.emit(format!(
                "{} {}_{};",
                sym.r_type.to_c_type(),
                f.name,
                sym.name
            ));
        }
        self.emit(format!(
            "{} {};",
            f.r_type.to_c_type(),
            Compiler::f_ret_value(f.name.clone())
        ));
        self.emit(format!("void* {};", Compiler::f_ret_ptr(f.name.clone())));
        self.emit(format!("void* fptr_{} = &&f_{};", f.name, f.name));
        self.emit_label(format!("f_{}", f.name));
        self.compile_ast(*f.block);
        self.emit(format!("goto *{};", Compiler::f_ret_ptr(f.name)))
    }

    pub fn type_for_last(r_type: Type) -> String {
        match r_type {
            Type::Simple(a) => match a {
                SimpleType::String => "str".to_string(),
                _ => a.to_c_type(),
            },
            Type::Array(a) => match a {
                SimpleType::String => "str_arr".to_string(),
                _ => format!("{}_arr", a.to_c_type()),
            },
        }
    }

    pub fn compile_function_call(&mut self, f: FunctionCallNode) {
        trace!("Compiling function call");
        let label = self.advance_label();
        for arg in f.args.iter() {
            self.compile_ast(arg.1.clone());
            let r_type = Compiler::type_for_last(arg.1.r_type());
            self.emit(format!("{}_{} = last_{};", f.target, arg.0, r_type));
        }
        self.emit(format!(
            "{} = &&ret_{};",
            Compiler::f_ret_ptr(f.clone().target),
            label
        ));
        self.emit(format!("goto f_{};", f.target));
        self.emit_label(format!("ret_{}", label));
        self.emit(format!(
            "last_{} = {};",
            Compiler::type_for_last(f.r_type),
            Compiler::f_ret_value(f.target)
        ));
    }
}
