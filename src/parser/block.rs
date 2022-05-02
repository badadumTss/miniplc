use log::trace;

use crate::core::{
    ast::{ASTNode, BlockNode},
    errors::SyntaxError,
    token::Kind,
};

use super::Parser;

impl Parser {
    pub fn parse_block(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse block");
        let start = self.current.clone();

        let mut stmts: Vec<ASTNode> = Vec::new();
        let mut errors: Vec<SyntaxError> = Vec::new();

        self.advance();
        while !self.matches(Kind::End) && !self.matches(Kind::Eof) {
            if !self.matches(Kind::Begin) {
                self.go_back();
            }
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(errs) => {
                    for err in errs {
                        errors.push(err);
                    }
                    self.syncronize();
                }
            }
            self.advance();
        }

        let symbols = self.context.pop().unwrap();

        if errors.is_empty() {
            Ok(ASTNode::Block(BlockNode {
                position: start.position,
                context: symbols,
                statements: stmts.into_boxed_slice(),
            }))
        } else {
            Err(errors)
        }
    }
}
