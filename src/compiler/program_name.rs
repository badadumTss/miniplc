use crate::core::ast::ProgramNameNode;

use super::Compiler;

impl Compiler {
    /// Compiles the program name as an output that states the program
    /// name
    pub fn compile_program_name(&mut self, node: ProgramNameNode) {
        self.emit(format!("printf({});", node.name.lexeme));
    }
}
