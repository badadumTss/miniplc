use crate::core::ast::ProgramNode;

use super::Compiler;

impl Compiler {
    pub fn compile_program(&mut self, node: ProgramNode) {
        self.emit("int last_int;".to_string());
        self.emit("float last_float;".to_string());
        self.emit("bool last_bool;".to_string());
        self.emit("char* last_str;".to_string());
        self.emit("int* last_int_arr;".to_string());
        self.emit("float* last_float_arr;".to_string());
        self.emit("bool last_bool_arr;".to_string());
        self.emit("char** last_str_arr;".to_string());
        self.emit("void* main_block_ptr = &&main_block;".to_string());
        self.emit("goto *main_block_ptr;".to_string());

        for f in node.functions.iter() {
            self.compile_function(f.clone());
        }

        for p in node.procedures.iter() {
            self.compile_procedure(p.clone());
        }

        self.scope = "main".to_string();
        self.emit_label("main_block".to_string());
        self.compile_block(node.main_block);
        self.emit("return 0;".to_string());
    }
}
