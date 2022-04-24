use crate::{
    core::{ast::*, errors::SyntaxError, token::Kind, variable::Type},
    parser::Parser,
    scanner::position::Position,
};

impl Parser {
    fn declare_variable(
        &mut self,
        lexeme: String,
        ttype: Type,
        position: Position,
    ) -> Result<(), SyntaxError> {
        match self.context.insert(lexeme.clone(), (ttype, position)) {
	    Some(v) => Err(SyntaxError {
		position: self.current.position,
		raw_line: self.scanner.curr_line(),
		description:
		format!("Declaration of an already declared variable: ({}, {}), previously declared at {}",
			lexeme, v.0, v.1)
	    }),
	    None => Ok(())
	}
    }

    /// Parse function for a variable declaration, returns either an
    /// ASTNode rapresenting the statement or a syntax error, that has
    /// to be processed by the caller function
    pub fn parse_var_declaration(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        match self.advance().kind {
            Kind::Identifier => {
                let id = self.current.clone();
                match self.advance().kind {
                    Kind::Colon => match self.advance().kind {
                        Kind::Type => {
                            let var_type = match self.current.lexeme.as_str() {
                                "int" => Type::Integer,
                                "bool" => Type::Boolean,
                                _ => Type::String,
                            };
                            // Declaring a varible here allows to give
                            // better errors, user might have done an
                            // erroneous declaration of a variable but
                            // use it in subsequent statements
                            match self.declare_variable(
                                id.clone().lexeme,
                                var_type.clone(),
                                id.position,
                            ) {
                                Ok(()) => match self.advance().kind {
                                    Kind::Semicolon => {
                                        Ok(ASTNode::VariableDecl(VariableDeclNode {
                                            position: self.current.position,
                                            id,
                                            var_type,
                                            value: None,
                                        }))
                                    }
                                    Kind::ColonEqual => {
                                        match self.parse_expression() {
                                            Ok(node) => match self.current.kind {
                                                Kind::Semicolon => {
                                                    Ok(ASTNode::VariableDecl(VariableDeclNode {
                                                        position: self.previous.position,
                                                        id,
                                                        var_type,
                                                        value: Some(ExpressionStmtNode {
                                                            position: self.previous.position,
                                                            child: Box::new(node),
                                                        }),
                                                    }))
                                                }
                                                other => Err(self
                                                    .unexpected_token_err(Kind::Semicolon, other)),
                                            },
                                            Err(synerr) => Err(synerr),
                                        }
                                    }
                                    other => Err(self.unexpected_token_err(Kind::Semicolon, other)),
                                },

                                Err(e) => Err(vec![e]),
                            }
                        }
                        other => Err(self.unexpected_token_err(Kind::Type, other)),
                    },
                    other => Err(self.unexpected_token_err(Kind::Colon, other)),
                }
            }
            other => Err(self.unexpected_token_err(Kind::Identifier, other)),
        }
    }
}
