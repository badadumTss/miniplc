use crate::core::{
    ast::{ASTNode, VarNameNode, VarReassignmentExprNode, VariableDeclNode},
    symbol_table::SymbolType,
};

use super::Compiler;

impl Compiler {
    pub fn compile_var_assignment(&mut self, expr: VarReassignmentExprNode) {
        self.compile_ast(expr.new_value.as_ref().clone());
        let name = self.get_symbol(expr.variable_to_reassign.as_ref().clone());
        self.push_instruction(format!(
            "{} = last_{};",
            name,
            Compiler::type_for_last(expr.new_value.r_type())
        ))
    }

    pub fn get_symbol(&mut self, item: ASTNode) -> String {
        match item.clone() {
            ASTNode::VarName(node) => match node.s_type {
                SymbolType::Var | SymbolType::Param | SymbolType::VarParam => {
                    node.id.lexeme.to_lowercase()
                }
                SymbolType::Arr => todo!(),
                _ => {
                    self.push_c_error(item, "Expected variable name, found function name");
                    String::new()
                }
            },
            ASTNode::ArrayRef(node) => {
                self.compile_ast(node.index.as_ref().clone());
                format!("{}[last_int]", node.array.lexeme)
            }
            _ => {
                self.push_c_error(item, "Unknown variable to reassign");
                "".to_string()
            }
        }
    }

    pub fn compile_var_decl(&mut self, expr: VariableDeclNode) {
        self.push_instruction(format!(
            "{} {};",
            expr.var_type.to_c_type(),
            expr.id.lexeme.to_lowercase()
        ));
    }

    pub fn compile_var_name(&mut self, expr: VarNameNode) {
        let name = match expr.s_type {
            SymbolType::Var | SymbolType::VarParam | SymbolType::Param => {
                expr.id.lexeme.to_lowercase()
            }
            SymbolType::Arr => todo!(),
            _ => {
                self.push_c_error(
                    ASTNode::VarName(expr.clone()),
                    "Expected variable name, found function name",
                );
                String::new()
            }
        };
        self.push_instruction(format!(
            "last_{} = {};",
            Compiler::type_for_last(expr.r_type),
            name
        ));
    }
}
