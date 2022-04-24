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
        let id = self.current.clone(); // the current token is the identifier of the variable
        match self.context.get(&id.lexeme) {
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
            None => Err(vec![SyntaxError::new(
                id.position,
                self.scanner.curr_line(),
                format!("Use of undeclared variable {}", id.lexeme),
            )]),
        }
    }
}
