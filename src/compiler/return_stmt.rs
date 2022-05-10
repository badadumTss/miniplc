use crate::core::ast::ReturnStmtNode;

use super::Compiler;

impl Compiler {
    pub fn compile_return(&mut self, expr: ReturnStmtNode) {
        if let Some(var) = expr.value {
            self.compile_ast(var.as_ref().clone());
            self.push_instruction(format!(
                "return last_{};",
                Compiler::type_for_last(var.r_type())
            ));
        } else {
            self.push_instruction("return;".to_string());
        }
    }
}
