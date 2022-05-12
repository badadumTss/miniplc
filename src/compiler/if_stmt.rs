use log::trace;

use crate::core::ast::{ElseStmtNode, IfStmtNode};

use super::Compiler;

impl Compiler {
    pub fn compile_if(&mut self, node: IfStmtNode) {
        trace!("compiling if statement");
        let cur = self.advance_label();

        self.emit(format!("void* thenptr_{} = &&then_{};", cur, cur));
        self.emit(format!("void* endifptr_{} = &&endif_{};", cur, cur));

        self.compile_ast(node.guard.as_ref().clone());
        self.emit(format!("if (last_bool) goto *thenptr_{};", cur));

        if let Some(else_node) = node.else_stmt {
            self.compile_ast(else_node.as_ref().clone());
        }
        self.emit(format!("goto *endifptr_{};", cur));
        self.emit_label(format!("then_{}", cur));
        self.compile_ast(node.then.as_ref().clone());
        self.emit_label(format!("endif_{}", cur));
    }

    pub fn compile_else(&mut self, node: ElseStmtNode) {
        self.compile_ast(node.block.as_ref().clone());
    }
}
