use crate::{
    core::{
        ast::{ASTNode, ForStmtNode},
        errors::SyntaxError,
        token::Kind,
    },
    parser::Parser,
};

impl Parser {
    fn for_end(&mut self) -> bool {
        match self.advance().kind {
            Kind::End => match self.advance().kind {
                Kind::For => true,
                _ => {
                    self.go_back();
                    false
                }
            },
            Kind::Eof => true,
            _ => {
                self.go_back();
                false
            }
        }
    }

    fn parse_statements_for(&mut self) -> Result<Vec<ASTNode>, Vec<SyntaxError>> {
        let mut statements = Vec::new();
        let mut errors: Vec<SyntaxError> = Vec::new();

        while !self.for_end() {
            match self.parse_statement() {
                Ok(node) => statements.push(node),
                Err(e) => {
                    for err in e {
                        errors.push(err);
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    pub fn parse_for_loop(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        let var_id = self.advance();
        match var_id.kind {
            Kind::Identifier => match self.context.get(&var_id.lexeme) {
                Some(_) => match self.advance().kind {
                    Kind::In => match self.parse_expression() {
                        Ok(ASTNode::ExpressionStmt(expr_from)) => match self.current.kind {
                            Kind::Ddot => match self.parse_expression() {
                                Ok(ASTNode::ExpressionStmt(expr_to)) => match self.current.kind {
                                    Kind::Do => match self.parse_statements_for() {
                                        Ok(statements) => match self.advance().kind {
                                            Kind::Semicolon => Ok(ASTNode::ForStmt(ForStmtNode {
                                                position: self.current.position,
                                                increment: var_id,
                                                range_start: Box::new(expr_from),
                                                range_end: Box::new(expr_to),
                                                statements: statements.into_boxed_slice(),
                                            })),
                                            other => {
                                                Err(self
                                                    .unexpected_token_err(Kind::Semicolon, other))
                                            }
                                        },
                                        Err(e) => Err(e),
                                    },
                                    other => Err(self.unexpected_token_err(Kind::Do, other)),
                                },
                                Ok(other) => Err(vec![SyntaxError {
                                    description: format!("Expected expression, found {}", other),
                                    position: self.previous.position,
                                    raw_line: self.scanner.curr_line(),
                                }]),
                                Err(synerr) => Err(synerr),
                            },
                            other => Err(self.unexpected_token_err(Kind::Ddot, other)),
                        },
                        Ok(other) => Err(vec![SyntaxError {
                            description: format!("Expected expression, found {}", other),
                            position: self.previous.position,
                            raw_line: self.scanner.curr_line(),
                        }]),
                        Err(synerr) => Err(synerr),
                    },
                    other => Err(self.unexpected_token_err(Kind::In, other)),
                },
                None => Err(vec![SyntaxError::new(
                    var_id.clone().position,
                    self.scanner.curr_line(),
                    format!(
                        "Use of undeclared variable {} as for loop incrementer",
                        var_id.clone().lexeme
                    ),
                )]),
            },
            other => Err(self.unexpected_token_err(Kind::Identifier, other)),
        }
    }
}
