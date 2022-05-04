use log::trace;

use crate::core::{
    ast::{ASTNode, LiteralExprNode},
    errors::SyntaxError,
    objects::Object,
    token::Kind,
    types::{SimpleType, Type},
};

use super::Parser;

impl Parser {
    pub fn parse_array(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing literal array");
        let mut symbols: Vec<ASTNode> = Vec::new();
        let mut errors: Vec<SyntaxError> = Vec::new();
        let mut to_return: Vec<Object> = Vec::new();
        let initial = self.current.clone();
        let mut r_type = SimpleType::Int;
        trace!(
            "Mannaggia dio, token: {}, al posto {}",
            self.current.lexeme,
            self.current.position
        );
        if self.matches(Kind::LeftSquare) {
            while !self.matches(Kind::RightSquare) {
                let sym = self.parse_unary()?;
                r_type = sym.r_type().internal();
                symbols.push(sym);
                if !self.matches(Kind::Comma) && !self.matches(Kind::RightSquare) {
                    return Err(vec![self.error_at_current(
                        format!("Unexpected token: {}", self.current.lexeme).as_str(),
                    )]);
                }
            }
            for sym in symbols.iter() {
                match sym {
                    ASTNode::Literal(l) => match l.r_type {
                        Type::Simple(t) => {
                            if t == r_type {
                                to_return.push(l.clone().value);
                            } else {
                                errors.push(self.error_at_current(
                                    "Inconsistent types in literal vector declaration",
                                ));
                            }
                        }
                        other => errors.push(self.error_at_current(
                            format!("Unexpected type of value: {}", other).as_str(),
                        )),
                    },
                    other => {
                        errors.push(self.error_at_current(
                            format!("Unexpected array value: {}", other).as_str(),
                        ))
                    }
                }
            }
        };

        trace!(
            "Mannaggia dio, token: {}, al posto {}",
            self.current.lexeme,
            self.current.position
        );
        if errors.is_empty() {
            Ok(ASTNode::Literal(LiteralExprNode {
                position: initial.position,
                value: Object::Array(to_return.into_boxed_slice()),
                r_type: Type::Array(r_type),
            }))
        } else {
            Err(errors)
        }
    }
}
