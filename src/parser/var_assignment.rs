use log::trace;

use crate::{
    core::{
        ast::{ASTNode, ExpressionStmtNode, VarReassignmentExprNode},
        errors::SyntaxError,
        token::Kind,
    },
    parser::Parser,
};

impl Parser {
    /// Parser function for a variable assignment, reutrns an ASTNode
    /// that rapresents this statement
    pub fn parse_var_assignment(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing var assignment");
        let id = self.current.clone(); // the current token is the identifier of the variable
        match self.context.last() {
            Some(table) => match table.get(id.lexeme.clone()) {
                Some(_) => match self.advance().kind {
                    Kind::ColonEqual => match self.parse_expression() {
                        Ok(node) => Ok(ASTNode::VarReassignment(VarReassignmentExprNode {
                            position: id.position,
                            variable_to_reassign: id,
                            new_value: ExpressionStmtNode {
                                position: self.current.position,
                                child: Box::new(node),
                            },
                        })),
                        Err(error) => Err(error),
                    },
                    other => Err(self.unexpected_token_err(Kind::ColonEqual, other)),
                },
                None => Err(vec![self.error_at_current(
                    format!("Use of undeclared variable {}", id.lexeme).as_str(),
                )]),
            },
            None => Err(vec![self.error_at_current(
                format!(
                    "Using variables in global scope is not permitted {}",
                    id.lexeme
                )
                .as_str(),
            )]),
        }
    }
}
