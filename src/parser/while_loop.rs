use log::trace;

use crate::{
    advance_with_expected,
    core::{
        ast::{ASTNode, WhileStmtNode},
        errors::SyntaxError,
        symbol_table::SymbolTable,
        token::Kind,
        types::SimpleType,
    },
    current_with_expected,
};

use super::Parser;

impl Parser {
    pub fn parse_while_loop(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        let while_token = self.current.clone();
        let expr = self.parse_expression()?;
        if expr.r_type().internal() == SimpleType::Bool {
            return current_with_expected!(
                Kind::Do,
                self,
                advance_with_expected!(Kind::Begin, self, {
                    self.context.push(SymbolTable::new());
                    let block = self.parse_block()?;
                    Ok(ASTNode::WhileStmt(WhileStmtNode {
                        position: while_token.position,
                        guard: Box::new(expr),
                        block: Box::new(block),
                    }))
                })
            );
        };
        trace!(
            "U dumbass, return type of expression is set as {}, expression: {:?}",
            expr.r_type(),
            expr
        );
        Err(vec![self.error_at_current(
            "Expected boolean expression as while loop guard",
        )])
    }
}
