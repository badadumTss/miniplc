use log::trace;

use crate::core::{ast::ASTNode, errors::SyntaxError, symbol_table::SymbolType};

use super::Parser;

impl Parser {
    pub fn parse_symbol(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing symbol");
        let scope = self.context.last().unwrap();
        match scope.get(self.current.lexeme.clone()) {
            Some(symbol) => match symbol.s_type {
                SymbolType::Function => self.parse_function_call(),
                SymbolType::Procedure => self.parse_procedure_call(),
                _ => self.parse_var_assignment(),
            },
            None => {
                trace!("ctx depth: {}, table: {}", self.context.len(), scope);
                Err(vec![self.error_at_current(
                    format!("Unknown symbol: {}", self.current.lexeme).as_str(),
                )])
            }
        }
    }
}
