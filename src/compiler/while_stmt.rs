use crate::core::ast::WhileStmtNode;

use super::Compiler;

impl Compiler {
    pub fn compile_while(&mut self, node: WhileStmtNode) {
        let label = self.advance_label();
        self.emit(format!(
            "void* endwhile_ptr_{} = &&endwhile_{};",
            label, label
        ));
        self.emit(format!("void* guard_ptr_{} = &&guard_{};", label, label));
        self.emit_label(format!("guard_{}", label));
        self.compile_ast(node.guard.as_ref().clone());
        self.emit(format!("if (!last_bool) goto *endwhile_ptr_{};", label));
        self.compile_ast(node.block.as_ref().clone());
        self.emit(format!("goto *guard_ptr_{};", label));
        self.emit_label(format!("endwhile_{}", label));
    }
}
