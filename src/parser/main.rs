use log::trace;

use crate::{
    core::{ast::ASTNode, errors::SyntaxError},
    parser::Parser,
};

impl Parser {
    pub fn parse_main_block(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsiong main block");
        self.parse_block()
    }
}
