use log::trace;

use crate::{
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
        match self.advance().kind {
            Kind::Identifier => match self.context.last() {
                Some(table) => match table.get(self.current.clone().lexeme) {
                    Some(_) => match self.advance().kind {
                        Kind::Semicolon => Ok(ASTNode::ReadStmt(ReadStmtNode {
                            position: pos,
                            variable_to_read_in: self.previous.clone(),
                        })),
                        other => Err(self.unexpected_token_err(Kind::Semicolon, other)),
                    },
                    None => Err(vec![SyntaxError::new(
                        self.current.position,
                        self.scanner.curr_line(),
                        format!(
                            "Trying to read into an uninitialized variable: {}",
                            self.current.lexeme
                        ),
                    )]),
                },
                None => Err(vec![SyntaxError::new(
                    self.current.position,
                    self.scanner.curr_line(),
                    "Found read statement in global scope".to_string(),
                )]),
            },

            other => Err(self.unexpected_token_err(Kind::Identifier, other)),
        }
    }
}
