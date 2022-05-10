use crate::core::ast::ProgramNode;

use super::Compiler;

impl Compiler {
    pub fn compile_program(&mut self, node: ProgramNode) {
        self.push_instruction("int last_int;".to_string());
        self.push_instruction("bool last_bool;".to_string());
        self.push_instruction("char* last_str;".to_string());
        self.push_instruction("int* last_int_arr;".to_string());
        self.push_instruction("bool last_bool_arr;".to_string());
        self.push_instruction("char** last_str_arr;".to_string());

        for f in node.functions.iter() {
            self.compile_function(f.clone());
        }

        for p in node.procedures.iter() {
            self.compile_procedure(p.clone());
        }

        self.scope = "main".to_string();
        self.compile_block(node.main_block);
        self.push_instruction("return 0;".to_string());
    }
}
