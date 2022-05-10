use log::trace;

use crate::{
    core::{
        ast::{ASTNode, ReadStmtNode},
        errors::SyntaxError,
    },
    parser::Parser,
};

impl Parser {
    /// Parses a read statement, returning the corresponding ASTNode
    pub fn parse_read(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing read statemet");
        let pos = self.current.clone().position;
        let where_to_read = self.parse_symbol()?;
        Ok(ASTNode::ReadStmt(ReadStmtNode {
            position: pos,
            variable_to_read_in: Box::new(where_to_read),
        }))
    }
}
