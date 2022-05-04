use log::trace;

use crate::{
    advance_with_expected,
    core::{ast::*, errors::SyntaxError, symbol_table::*, token::Kind, types::Type},
    parser::Parser,
    scanner::position::Position,
};

impl Parser {
    /// Declares a variable in current scope, being a function,
    /// procedure or main block scope, fails if already declared for
    /// this scope
    fn declare_variable(
        &mut self,
        lexeme: String,
        r_type: Type,
        position: Position,
    ) -> Result<(), Vec<SyntaxError>> {
        trace!("declaring variable {} of type {}", lexeme, r_type);
        match self.context.pop() {
            Some(mut table) => match table.get(lexeme.clone()) {
                Some(v) => Err(vec![self.error_at_current(
                    format!(
			"Declaration of an already declared variable: {}, previously declared at {}",
			lexeme, v.position
                    )
                    .as_str(),
                )]),
                None => {
                    table.push(Symbol {
                        name: lexeme,
                        s_type: match r_type {
                            Type::Array(_) => SymbolType::Arr,
                            Type::Simple(_) => SymbolType::Var,
                        },
                        r_type,
                        position,
                    });
                    self.context.push(table);
                    Ok(())
                }
            },
            None => Err(vec![self.error_at_current(
                "Declaring variable in global scope is not permitted",
            )]),
        }
    }

    /// Parse function for a variable declaration, returns either an
    /// ASTNode rapresenting the statement or a syntax error, that has
    /// to be processed by the caller function
    pub fn parse_var_declaration(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing var declaration");
        advance_with_expected!(Kind::Identifier, self, {
            let id = self.current.clone();
            advance_with_expected!(Kind::Colon, self, {
                let var_type = self.parse_type()?;
                advance_with_expected!(Kind::Semicolon, self, {
                    self.declare_variable(id.clone().lexeme, var_type, id.position)?;
                    Ok(ASTNode::VariableDecl(VariableDeclNode {
                        position: id.position,
                        id,
                        var_type,
                        value: None,
                    }))
                })
            })
        })
    }
}
