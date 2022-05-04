use log::trace;

use crate::advance_with_expected;
use crate::core::ast::{EofNode, ProgramNameNode};
use crate::core::symbol_table::{SymbolTable, SymbolType};
use crate::core::{ast::ASTNode, errors::SyntaxError, token::Kind};
use crate::parser::Parser;

impl Parser {
    pub fn parse_program_name(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing program name");
        advance_with_expected!(Kind::Identifier, self, {
            let id = self.current.clone();

            advance_with_expected!(
                Kind::Semicolon,
                self,
                Ok(ASTNode::ProgramName(ProgramNameNode { name: id }))
            )
        })
    }

    pub fn parse_global_statement(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing global statement");
        match self.advance().kind {
            Kind::Function => self.parse_function(),
            Kind::Procedure => self.parse_procedure(),
            Kind::Begin => {
                self.context.push(SymbolTable::new());
                let block = self.parse_main_block()?;
                self.context.pop();
                Ok(block)
            }
            Kind::Program => self.parse_program_name(),
            Kind::Eof => Ok(ASTNode::EofStmt(EofNode {
                eof: self.current.clone(),
            })),
            _ => {
                self.go_back();
                let stmt = self.parse_statement()?;
                Err(vec![self.error_at_current(
                    format!(
                        "Expected either function or procedure initialization, found {}",
                        stmt
                    )
                    .as_str(),
                )])
            }
        }
    }

    pub fn parse_statement(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing statement");
        match self.advance().kind {
            Kind::Var => self.parse_var_declaration(),
            Kind::Identifier => match self.get_symbol(self.current.lexeme.clone()) {
                Some(s) => match s.s_type {
                    SymbolType::Function => {
                        let node = self.parse_function_call()?;
                        advance_with_expected!(Kind::Semicolon, self, Ok(node))
                    }
                    SymbolType::Procedure => self.parse_procedure_call(),
                    _ => self.parse_var_assignment(),
                },
                None => Err(vec![self.error_at_current(
                    format!("Symbol not found: {}", self.current.lexeme).as_str(),
                )]),
            },
            Kind::Read => self.parse_read(),
            Kind::Print => self.parse_print(),
            Kind::Assert => self.parse_assert(),
            Kind::Return => self.parse_return(),
            Kind::While => self.parse_while_loop(),
            other => Err(vec![
                self.error_at_current(&format!("Unexpected token: {}", other))
            ]),
        }
    }

    /// Removes next statements, called by the parser to recover from
    /// an error in the current statement and go to the next one
    pub fn syncronize(&mut self) {
        while !self.matches(Kind::Semicolon)
            && !self.matches(Kind::Eof)
            && !self.matches(Kind::End)
            && !self.matches(Kind::Function)
            && !self.matches(Kind::Procedure)
            && !self.matches(Kind::Begin)
        {
            self.advance();
        }

        if self.matches(Kind::Function)
            || self.matches(Kind::Procedure)
            || self.matches(Kind::Begin)
        {
            self.go_back();
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
            raw_line: self.scanner.curr_line(),
            description: format!("Expected token: {}, found {}", expected_token, found_token),
        }]
    }
}
