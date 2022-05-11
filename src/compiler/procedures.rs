use log::trace;

use crate::core::ast::{ProcedureCallNode, ProcedureDeclNode};

use super::Compiler;

impl Compiler {
    pub fn compile_procedure(&mut self, f: ProcedureDeclNode) {
        trace!("Compiling procedure declaration");
        self.scope = f.name.clone();
        for sym in f.args.iter() {
            self.push_instruction(format!(
                "{} {}_{};",
                sym.r_type.to_c_type(),
                f.name,
                sym.name
            ));
        }
        self.push_instruction(format!("void* {};", Compiler::f_ret_ptr(f.name.clone())));
        self.push_instruction(format!("void* fptr_{} = &&f_{};", f.name, f.name));
        self.push_label(format!("f_{}", f.name));
        self.compile_ast(*f.block);
        self.push_instruction(format!("goto *{};", Compiler::f_ret_ptr(f.name)))
    }

    pub fn compile_procedure_call(&mut self, f: ProcedureCallNode) {
        trace!("Compiling procedure call");
        let label = self.advance_label();
        for (name, arg) in f.args.iter() {
            self.compile_ast(arg.clone());
            let r_type = Compiler::type_for_last(arg.r_type());
            self.push_instruction(format!("{}_{} = last_{};", f.target, name, r_type));
        }
        self.push_instruction(format!(
            "{} = &&ret_{};",
            Compiler::f_ret_ptr(f.target.clone()),
            label
        ));
        self.push_instruction(format!("goto f_{};", f.target));
        self.push_label(format!("ret_{}", label));
    }
}
