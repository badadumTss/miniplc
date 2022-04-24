use crate::core::errors::EvaluationError;

use crate::{
    core::{ast::ASTNode, variable::Variable},
    interpreter::Interpreter,
};

impl Interpreter {
    pub(crate) fn eval_statement(
        &mut self,
        stmt: ASTNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match stmt {
            ASTNode::BinaryExpression(node) => self.eval_expression_binary(node),
            ASTNode::Identifier(node) => self.eval_identifier(node),
            ASTNode::Literal(node) => self.eval_literal(node),
            ASTNode::UnaryExpression(node) => self.eval_expression_unary(node),
            ASTNode::VarReassignment(node) => self.eval_var_reassignment(node),
            ASTNode::VariableDecl(node) => self.eval_var_declaration(node),
            ASTNode::ExpressionStmt(node) => self.eval_expression_statement(node),
            ASTNode::ForStmt(node) => self.eval_for(node),
            ASTNode::PrintStmt(node) => self.eval_print(node),
            ASTNode::ReadStmt(node) => self.eval_read(node),
            ASTNode::AssertStmt(node) => self.eval_assert(node),
            ASTNode::EofStmt(_) => Ok(Variable::from_int(0)),
            node => self.eval_statement(node),
        }
    }
}
