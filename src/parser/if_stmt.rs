use log::trace;

use crate::{
    core::{
        ast::{ASTNode, ElseStmtNode, IfStmtNode},
        errors::SyntaxError,
        token::Kind,
        types::{SimpleType, Type},
    },
    current_with_expected,
};

use super::Parser;

impl Parser {
    pub fn parse_if(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing if statement");
        let if_id = self.current.clone();
        let guard = self.parse_expression()?;
        if guard.r_type() == Type::Simple(SimpleType::Bool) {
            current_with_expected!(Kind::Then, self, {
                let then = self.parse_statement()?;
                self.advance();
                if self.matches(Kind::Else) {
                    trace!("parsing else statement");
                    let else_id = self.current.clone();
                    let else_stmt = self.parse_statement()?;
                    Ok(ASTNode::IfStmt(IfStmtNode {
                        position: if_id.position,
                        guard: Box::new(guard),
                        then: Box::new(then),
                        else_stmt: Some(Box::new(ASTNode::ElseStmt(ElseStmtNode {
                            position: else_id.position,
                            block: Box::new(else_stmt),
                        }))),
                    }))
                } else {
                    self.go_back();
                    Ok(ASTNode::IfStmt(IfStmtNode {
                        position: if_id.position,
                        guard: Box::new(guard),
                        then: Box::new(then),
                        else_stmt: None,
                    }))
                }
            })
        } else {
            Err(vec![self.error_at_current(
                "Expected boolean expression as if guard",
            )])
        }
    }
}
