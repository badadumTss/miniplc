use log::trace;

use crate::{
    core::{
        ast::{ASTNode, ArrayRefExpr, VarNameNode},
        errors::SyntaxError,
        symbol_table::SymbolType,
        token::Kind,
        types::Type,
    },
    current_with_expected,
};

use super::Parser;

impl Parser {
    pub fn parse_symbol(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing symbol");
        let id = self.current.clone();
        trace!(
            "parsing symbol {:?}",
            self.get_symbol(self.current.lexeme.clone())
        );
        match self.get_symbol(self.current.lexeme.clone()) {
            Some(symbol) => match symbol.s_type {
                SymbolType::Function => self.parse_function_call(),
                SymbolType::Procedure => self.parse_procedure_call(),
                SymbolType::Arr => match self.advance().kind {
                    Kind::LeftSquare => {
                        let xpr = self.parse_expression()?;
                        current_with_expected!(
                            Kind::RightSquare,
                            self,
                            Ok(ASTNode::ArrayRef(ArrayRefExpr {
                                position: id.position,
                                array: id,
                                index: Box::new(xpr),
                                r_type: Type::Simple(symbol.r_type.internal()),
                            }))
                        )
                    }
                    Kind::Semicolon => {
                        self.go_back();
                        Ok(ASTNode::VarName(VarNameNode {
                            position: id.position,
                            id,
                            r_type: symbol.r_type,
                            s_type: symbol.s_type,
                        }))
                    }
                    other => Err(vec![self.error_at_current(
                        format!(
                            "Expected either array referencing or nothing, found {}",
                            other
                        )
                        .as_str(),
                    )]),
                },
                _ => Ok(ASTNode::VarName(VarNameNode {
                    position: self.current.position,
                    id: self.current.clone(),
                    r_type: symbol.r_type,
                    s_type: symbol.s_type,
                })),
            },
            None => Err(vec![self.error_at_current(
                format!("Unknown symbol: {}", self.current.lexeme).as_str(),
            )]),
        }
    }
}
