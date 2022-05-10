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
        let mut i = 1;
        for sym in f.args.iter() {
            self.push_instruction(format!("{} param{}_{};", sym.r_type.to_c_type(), i, f.name));
            i += 1;
        }
        self.push_instruction(format!(
            "{} {};",
            f.r_type.to_c_type(),
            Compiler::f_ret_value(f.name.clone())
        ));
        self.push_instruction(format!("void* {};", Compiler::f_ret_ptr(f.name.clone())));
        self.push_instruction(format!("void* fptr_{} = &&f_{};", f.name, f.name));
        self.push_instruction(format!("f_{}:", f.name));
        i = 1;
        for sym in f.args.iter() {
            self.push_instruction(format!(
                "{} param{}_{} = param{}_{};",
                sym.r_type.to_c_type(),
                sym.name,
                f.name,
                i,
                f.name
            ));
            i += 1;
        }
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
                SimpleType::Int => "int_ptr",
                SimpleType::String => "str_ptr",
                SimpleType::Bool => "bool_ptr",
                SimpleType::Void => "void_ptr",
            },
        }
    }

    pub fn compile_function_call(&mut self, f: FunctionCallNode) {
        trace!("Compiling function call");
        let label = self.advance_label();
        let mut i = 1;
        for arg in f.args.iter() {
            self.compile_ast(arg.clone());
            let r_type = Compiler::type_for_last(arg.r_type());
            self.push_instruction(format!("param{}_{} = last_{};", i, f.target, r_type));
            i += 1;
        }
        self.push_instruction(format!(
            "{} = &&ret_{};",
            Compiler::f_ret_ptr(f.clone().target),
            label
        ));
        self.push_instruction(format!("goto *fptr_{};", f.target));
        self.push_instruction(format!("ret_{}:", label));
        self.push_instruction(format!(
            "last_{} = {};",
            Compiler::type_for_last(f.r_type),
            Compiler::f_ret_value(f.target)
        ));
    }
}
