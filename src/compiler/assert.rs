use log::trace;

use crate::core::ast::AssertStmtNode;

use super::Compiler;

impl Compiler {
    /// Compiles an assert node, runs `assert` from c std
    pub fn compile_assert(&mut self, node: AssertStmtNode) {
        trace!("compiling assert stmt");
        self.compile_ast(node.expr.as_ref().clone());
        self.emit("assert(last_bool);".to_string());
    }
}
