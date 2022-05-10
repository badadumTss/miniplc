use log::trace;

use crate::core::ast::BlockNode;

use super::Compiler;

impl Compiler {
    pub fn compile_block(&mut self, block: BlockNode) {
        trace!("compiling block of statements");
        for stmt in block.statements.iter() {
            self.compile_ast(stmt.clone());
        }
    }
}
