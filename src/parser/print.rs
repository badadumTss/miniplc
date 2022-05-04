use log::trace;

use crate::{
    advance_with_expected,
    core::{
        ast::{ASTNode, PrintStmtNode},
        errors::SyntaxError,
        token::Kind,
    },
    current_with_expected,
    parser::Parser,
};

impl Parser {
    /// Parses a print statement returning the corresponding ASTNode
    pub fn parse_print(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing print statement");
        let position = self.current.clone().position;
        advance_with_expected!(Kind::LeftParen, self, {
            let expr = self.parse_expression()?;
            current_with_expected!(
                Kind::RightParen,
                self,
                advance_with_expected!(
                    Kind::Semicolon,
                    self,
                    Ok(ASTNode::PrintStmt(PrintStmtNode {
                        to_print: Box::new(expr),
                        position,
                    }))
                )
            )
        })
    }
}
