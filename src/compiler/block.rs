use log::trace;

use crate::core::ast::BlockNode;

use super::Compiler;

impl Compiler {
    /// Compiles a block, basically just recursively compiles the
    /// innser statements since the scoping is provided by the
    /// variable names
    pub fn compile_block(&mut self, block: BlockNode) {
        trace!("compiling block of statements");
        for stmt in block.statements.iter() {
            self.compile_ast(stmt.clone());
        }
    }
}
