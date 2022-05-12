use crate::core::{
    ast::{ASTNode, VarNameNode, VarReassignmentExprNode, VariableDeclNode},
    symbol_table::SymbolType,
};

use super::Compiler;

impl Compiler {
    pub fn compile_var_assignment(&mut self, expr: VarReassignmentExprNode) {
        match expr.variable_to_reassign.as_ref().clone() {
            ASTNode::VarName(node) => match node.s_type {
                SymbolType::Var | SymbolType::VarParam | SymbolType::Param | SymbolType::Arr => {
                    let name = format!("{}_{}", self.scope, node.id.lexeme.to_lowercase());
                    self.compile_ast(expr.new_value.as_ref().clone());
                    self.emit(format!(
                        "{} = last_{};",
                        name,
                        Compiler::type_for_last(expr.new_value.r_type())
                    ));
                }
                _ => {
                    self.push_c_error(
                        expr.variable_to_reassign.as_ref().clone(),
                        "Expected variable name, found function name",
                    );
                }
            },
            ASTNode::ArrayRef(node) => {
                let label = self.advance_label();
                self.compile_ast(node.index.as_ref().clone());
                self.emit(format!("int tmp_{} = last_int;", label));
                self.compile_ast(expr.new_value.as_ref().clone());
                let name = format!("{}_{}[tmp_{}]", self.scope, node.array.lexeme, label);
                self.emit(format!(
                    "{} = last_{};",
                    name,
                    Compiler::type_for_last(expr.new_value.r_type())
                ))
            }
            _ => {
                self.push_c_error(
                    expr.variable_to_reassign.as_ref().clone(),
                    "Unknown variable to reassign",
                );
            }
        }
    }

    pub fn compile_var_decl(&mut self, expr: VariableDeclNode) {
        self.emit(format!(
            "{} {}_{};",
            expr.var_type.to_c_type(),
            self.scope,
            expr.id.lexeme.to_lowercase()
        ));
    }

    pub fn compile_var_name(&mut self, expr: VarNameNode) {
        let name = match expr.s_type {
            SymbolType::Var | SymbolType::VarParam | SymbolType::Param | SymbolType::Arr => {
                format!("{}_{}", self.scope, expr.id.lexeme.to_lowercase())
            }
            _ => {
                self.push_c_error(
                    ASTNode::VarName(expr.clone()),
                    "Expected variable name, found function name",
                );
                String::new()
            }
        };
        self.emit(format!(
            "last_{} = {};",
            Compiler::type_for_last(expr.r_type),
            name
        ));
    }
}
