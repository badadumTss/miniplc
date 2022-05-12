use crate::core::ast::ProgramNameNode;

use super::Compiler;

impl Compiler {
    pub fn compile_program_name(&mut self, node: ProgramNameNode) {
        self.emit(format!("printf({});", node.name.lexeme));
    }
}
