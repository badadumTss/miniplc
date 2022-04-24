use crate::{
    core::{
        ast::{ASTNode, ExpressionStmtNode, PrintStmtNode},
        errors::SyntaxError,
        token::Kind,
    },
    parser::Parser,
};

impl Parser {
    /// Parses a print statement returning the corresponding ASTNode
    pub fn parse_print(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        let pos = self.current.clone().position;
        match self.parse_expression() {
            Ok(expr) => match self.current.kind {
                Kind::Semicolon => Ok(ASTNode::PrintStmt(PrintStmtNode {
                    to_print: ExpressionStmtNode {
                        position: self.previous.position,
                        child: Box::new(expr),
                    },
                    position: pos,
                })),
                other => Err(self.unexpected_token_err(Kind::Semicolon, other)),
            },

            Err(e) => Err(e),
        }
    }
}
