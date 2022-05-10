use crate::core::ast::ProgramNameNode;

use super::Compiler;

impl Compiler {
    pub fn compile_program_name(&mut self, node: ProgramNameNode) {
        self.push_instruction(format!("printf({});", node.name.lexeme));
    }
}
