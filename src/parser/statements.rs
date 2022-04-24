use crate::core::ast::EofNode;
use crate::core::{ast::ASTNode, errors::SyntaxError, token::Kind};
use crate::parser::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        match self.advance().kind {
            Kind::Var => self.parse_var_declaration(),
            Kind::Identifier => self.parse_var_assignment(),
            Kind::For => self.parse_for_loop(),
            Kind::Read => self.parse_read(),
            Kind::Print => self.parse_print(),
            Kind::Assert => self.parse_assert(),
            Kind::Semicolon => self.parse_statement(),
            Kind::Function => self.parse_function(),
            Kind::Procedure => self.parse_procedure(),
            Kind::Begin => self.parse_block(),
            Kind::Eof => Ok(ASTNode::EofStmt(EofNode {
                eof: self.current.clone(),
            })),
            _ => Err(vec![SyntaxError {
                position: self.current.position,
                raw_line: self.scanner.curr_line(),
                description: format!("Unexpected token: {}", self.current.lexeme),
            }]),
        }
    }

    /// Removes next statements, called by the parser to recover from
    /// an error in the current statement and go to the next one
    pub fn syncronize(&mut self) {
        while !self.matches(Kind::Semicolon) && !self.matches(Kind::Eof) {
            self.advance();
        }
    }

    /// Generates a syntax error for an unexpected token.
    pub fn unexpected_token_err(
        &mut self,
        expected_token: Kind,
        found_token: Kind,
    ) -> Vec<SyntaxError> {
        vec![SyntaxError {
            position: self.current.position,
            raw_line: self
                .scanner
                .line(self.current.position.line.try_into().unwrap()),
            description: format!("Expected token: {}, found {}", expected_token, found_token),
        }]
    }
}
