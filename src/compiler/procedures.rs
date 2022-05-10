use log::trace;

use crate::core::ast::{ProcedureCallNode, ProcedureDeclNode};

use super::Compiler;

impl Compiler {
    pub fn compile_procedure(&mut self, f: ProcedureDeclNode) {
        trace!("Compiling procedure declaration");
        self.scope = f.name.clone();
        let mut i = 1;
        for sym in f.args.iter() {
            self.push_instruction(format!("{} param{}_{};", sym.r_type.to_c_type(), i, f.name));
            i += 1;
        }
        self.push_instruction(format!("void* {};", Compiler::f_ret_ptr(f.name.clone())));
        self.push_instruction(format!("void* fptr_{} = && f_{};", f.name, f.name));
        self.push_instruction(format!("f_{}:", f.name));
        self.compile_ast(*f.block);
        self.push_instruction(format!("goto *{};", Compiler::f_ret_ptr(f.name)))
    }

    pub fn compile_procedure_call(&mut self, f: ProcedureCallNode) {
        trace!("Compiling procedure call");
        let label = self.advance_label();
        let mut i = 1;
        for arg in f.args.iter() {
            self.compile_ast(arg.clone());
            let r_type = Compiler::type_for_last(arg.r_type());
            self.push_instruction(format!("param{}_{} = last_{};", i, f.target, r_type));
            i += 1;
        }
        self.push_instruction(format!(
            "{} = &&ret_{}",
            Compiler::f_ret_ptr(f.target.clone()),
            label
        ));
        self.push_instruction(format!("goto *fptr_{};", f.target));
        self.push_instruction(format!("ret_{}:", label));
    }
}
