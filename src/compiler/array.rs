use log::trace;

use crate::core::ast::ArrayRefExpr;

use super::Compiler;

impl Compiler {
    pub fn compile_array_ref(&mut self, arr: ArrayRefExpr) {
        trace!("compiling array reference");
        self.compile_ast(arr.index.as_ref().clone());
        let r_type = arr.r_type.internal().to_c_type();
        let name = format!("{}_{}", self.scope, arr.array.lexeme);
        self.emit(format!("last_{} = {}[last_int];", r_type, name));
    }
}
