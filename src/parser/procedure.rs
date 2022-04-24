use crate::core::{ast::ASTNode, errors::SyntaxError};

use super::Parser;

impl Parser {
    pub fn parse_procedure(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        todo!()
    }
}
