use log::trace;

use crate::{
    advance_with_expected,
    core::{
        ast::{ASTNode, ReadStmtNode},
        errors::SyntaxError,
        token::Kind,
    },
    parser::Parser,
};

impl Parser {
    /// Parses a read statement, returning the corresponding ASTNode
    pub fn parse_read(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing read statemet");
        let pos = self.current.clone().position;
        advance_with_expected!(Kind::LeftParen, self, {
            self.advance();
            let where_to_read = self.parse_symbol()?;
            advance_with_expected!(
                Kind::RightParen,
                self,
                advance_with_expected!(
                    Kind::Semicolon,
                    self,
                    Ok(ASTNode::ReadStmt(ReadStmtNode {
                        position: pos,
                        variable_to_read_in: Box::new(where_to_read),
                    }))
                )
            )
        })
    }
}
