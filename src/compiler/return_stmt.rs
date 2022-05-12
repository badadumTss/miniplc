use crate::core::ast::ReturnStmtNode;

use super::Compiler;

impl Compiler {
    pub fn compile_return(&mut self, expr: ReturnStmtNode) {
        if let Some(var) = expr.value {
            self.compile_ast(var.as_ref().clone());
            self.emit(format!(
                "{} = last_{};",
                Compiler::f_ret_value(self.scope.clone()),
                Compiler::type_for_last(var.r_type())
            ));
        } else {
            self.emit("return;".to_string());
        }
    }
}
