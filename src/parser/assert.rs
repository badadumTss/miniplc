use log::trace;

use crate::{
    core::{
        ast::{ASTNode, AssertStmtNode},
        errors::SyntaxError,
        token::Kind,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_assert(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse assert");
        match self.advance().kind {
            Kind::LeftParen => match self.parse_expression() {
                Ok(expr) => match self.current.kind {
                    Kind::RightParen => match self.advance().kind {
                        Kind::Semicolon => Ok(ASTNode::AssertStmt(AssertStmtNode {
                            position: self.current.position,
                            expr: Box::new(expr),
                        })),
                        other => Err(self.unexpected_token_err(Kind::Semicolon, other)),
                    },
                    other => Err(self.unexpected_token_err(Kind::RightParen, other)),
                },
                Err(synerr) => Err(synerr),
            },
            other => Err(self.unexpected_token_err(Kind::LeftParen, other)),
        }
    }
}
