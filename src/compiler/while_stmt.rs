use crate::core::ast::WhileStmtNode;

use super::Compiler;

impl Compiler {
    pub fn compile_while(&mut self, node: WhileStmtNode) {
        let label = self.advance_label();
        self.push_instruction(format!(
            "void* endwhile_ptr_{} = &&endwhile_{};",
            label, label
        ));
        self.push_instruction(format!("void* guard_ptr_{} = &&guard_{};", label, label));
        self.push_instruction(format!("guard_{}:", label));
        self.compile_ast(node.guard.as_ref().clone());
        self.push_instruction(format!("if (!last_bool) goto *endwhile_ptr_{};", label));
        self.compile_ast(node.block.as_ref().clone());
        self.push_instruction(format!("goto *guard_ptr_{};", label));
        self.push_instruction(format!("endwhile_{}:", label));
    }
}
