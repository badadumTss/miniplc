use log::trace;

use crate::core::ast::ArrayRefExpr;

use super::Compiler;

impl Compiler {
    pub fn compile_array_ref(&mut self, arr: ArrayRefExpr) {
        trace!("compiling array reference");
        self.push_instruction(arr.array.lexeme + "[");
        self.compile_ast(arr.index.as_ref().clone());
        self.push_instruction("]".to_string());
    }
}
