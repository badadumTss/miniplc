use crate::core::{
    ast::{FunctionDeclNode, ProcedureDeclNode, ProgramNode},
    symbol_table::SymbolTable,
};

use super::Compiler;

impl Compiler {
    pub fn get_args(&self, mut args: SymbolTable) -> String {
        let mut to_return = String::new();
        match args.pop() {
            Some(sym) => {
                to_return = format!("{}", sym.r_type);
                for el in args.symbols.iter() {
                    to_return = format!("{}, {}", to_return, el.r_type);
                }
            }
            None => {}
        }
        to_return
    }

    pub fn push_function_sign(&mut self, node: FunctionDeclNode) {
        self.src_ctrl.push_instruction(format!(
            "{} {}({});\n\t",
            node.r_type,
            node.name,
            self.get_args(node.args)
        ));
    }

    pub fn push_procedure_sign(&mut self, node: ProcedureDeclNode) {
        self.src_ctrl.push_instruction(format!(
            "void {}({});\n\t",
            node.name,
            self.get_args(node.args)
        ));
    }

    pub fn compile_program(&mut self, node: ProgramNode) {
        for f in node.functions.iter() {
            self.push_function_sign(f.clone());
        }

        for p in node.procedures.iter() {
            self.push_procedure_sign(p.clone());
        }
    }
}
