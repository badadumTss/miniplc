use log::trace;

use crate::{
    core::{
        ast::*,
        errors::SyntaxError,
        objects::Object,
        symbol_table::SymbolType,
        token::Kind,
        types::{SimpleType, Type},
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_unary(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Ok, parsing a unary: {}", self.current.clone().lexeme);
        match self.advance().kind {
            Kind::Bang => {
                let bang = self.current.clone();
                match self.parse_unary() {
                    Ok(unary) => Ok(ASTNode::UnaryExpression(UnaryExprNode {
                        position: bang.position,
                        operand: bang,
                        expression: Box::new(unary),
                    })),
                    Err(synerr) => Err(synerr),
                }
            }
            Kind::Identifier => {
                trace!("found identifier: {}", self.current.clone().lexeme);
                match self.context.last().unwrap().get(&self.current.lexeme) {
		    Some(sym) => match sym.s_type {
			SymbolType::Function => self.parse_function_call(),
			SymbolType::Procedure => Err(vec![self.error_at_current("Procedures produce no value to return, cannot be used inside an expression")]),
			_ => {
			    trace!("found identifier {}", self.current.clone().lexeme);
			    self.advance();
			    Ok(ASTNode::Identifier(IdentifierExprNode {
				position: self.previous.position,
				id: self.previous.clone(),
			    }))
			},
		    }
		    None => Err(vec![self.error_at_current(format!("Unknown symbol in expression: {}", self.current.lexeme).as_str())])
		}
            }
            Kind::True => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Bool(true),
                    actual_type: Type::Simple(SimpleType::Bool),
                }))
            }
            Kind::False => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Bool(true),
                    actual_type: Type::Simple(SimpleType::Bool),
                }))
            }
            Kind::LitString => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::String(self.previous.clone().lexeme),
                    actual_type: Type::Simple(SimpleType::String),
                }))
            }
            Kind::LitInt => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Int(self.previous.clone().lexeme.parse::<i64>().unwrap()),
                    actual_type: Type::Simple(SimpleType::Int),
                }))
            }
            Kind::LeftParen => match self.parse_expression() {
                Ok(node) => match self.current.kind {
                    Kind::RightParen => {
                        self.advance();
                        Ok(ASTNode::ExpressionStmt(ExpressionStmtNode {
                            position: self.current.position,
                            child: Box::new(node),
                        }))
                    }
                    _ => Err(vec![self.error_at_current("Missing closing bracket")]),
                },
                Err(synerr) => Err(synerr),
            },
            other => {
                Err(vec![self.error_at_current(
                    format!("Unexpected token {:?}", other).as_str(),
                )])
            }
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse factor");
        match self.parse_unary() {
            Ok(left_factor) => match self.current.kind {
                Kind::Slash | Kind::Star => {
                    let op = self.current.clone();
                    match self.parse_unary() {
                        Ok(right_factor) => Ok(ASTNode::BinaryExpression(BinaryExprNode {
                            position: self.current.position,
                            left: Box::new(left_factor),
                            op: op.clone(),
                            op_type: match op.kind {
                                Kind::Slash => BinaryExprType::Division,
                                Kind::Star => BinaryExprType::Multiplication,
                                _ => panic!("Wtf? {:?}", op.kind),
                            },
                            right: Box::new(right_factor),
                        })),
                        Err(synerr) => Err(synerr),
                    }
                }
                _ => Ok(left_factor),
            },
            Err(synerr) => Err(synerr),
        }
    }

    fn parse_term(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse term");
        let left_term = self.parse_factor()?;
        match self.current.kind {
            Kind::Minus | Kind::Plus => {
                let op = self.current.clone();
                let right_term = self.parse_factor()?;
                Ok(ASTNode::BinaryExpression(BinaryExprNode {
                    position: self.current.position,
                    left: Box::new(left_term),
                    op: op.clone(),
                    op_type: match op.kind {
                        Kind::Minus => BinaryExprType::Subtraction,
                        Kind::Plus => BinaryExprType::Addition,
                        _ => panic!("Wtf? {:?}", op.kind),
                    },
                    right: Box::new(right_term),
                }))
            }
            _ => Ok(left_term),
        }
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse comparison");
        match self.parse_term() {
            Ok(left_term) => match self.current.kind {
                Kind::Greater | Kind::GreaterEqual | Kind::Less | Kind::LessEqual => {
                    let op = self.current.clone();
                    match self.parse_term() {
                        Ok(right_term) => Ok(ASTNode::BinaryExpression(BinaryExprNode {
                            position: self.current.position,
                            left: Box::new(left_term),
                            op: op.clone(),
                            op_type: match op.kind {
                                Kind::Greater => BinaryExprType::LogicGreaterThan,
                                Kind::GreaterEqual => BinaryExprType::LogicGreaterThanEQ,
                                Kind::Less => BinaryExprType::LogicLessThan,
                                Kind::LessEqual => BinaryExprType::LogicLessThanEQ,
                                _ => panic!("Wtf? {:?}", op.kind),
                            },
                            right: Box::new(right_term),
                        })),
                        Err(synerr) => Err(synerr),
                    }
                }
                _ => Ok(left_term),
            },
            Err(synerr) => Err(synerr),
        }
    }

    fn parse_equality(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse equality");
        match self.parse_comparison() {
            Ok(node_left) => match self.current.kind {
                Kind::Equal => {
                    let equal = self.current.clone();
                    match self.parse_comparison() {
                        Ok(node_right) => Ok(ASTNode::BinaryExpression(BinaryExprNode {
                            position: self.current.position,
                            left: Box::new(node_left),
                            op: equal,
                            op_type: BinaryExprType::LogicEQ,
                            right: Box::new(node_right),
                        })),
                        Err(err) => Err(err),
                    }
                }
                _ => Ok(node_left),
            },

            Err(err) => Err(err),
        }
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse expression");
        match self.parse_equality() {
            Ok(eq_left) => match self.current.kind {
                Kind::And => {
                    let equal = self.current.clone();
                    match self.parse_equality() {
                        Ok(eq_right) => {
                            let child = ASTNode::BinaryExpression(BinaryExprNode {
                                position: self.current.position,
                                left: Box::new(eq_left),
                                op: equal,
                                op_type: BinaryExprType::LogicAND,
                                right: Box::new(eq_right),
                            });
                            Ok(ASTNode::ExpressionStmt(ExpressionStmtNode {
                                position: self.current.position,
                                child: Box::new(child),
                            }))
                        }
                        Err(err) => Err(err),
                    }
                }
                _ => Ok(ASTNode::ExpressionStmt(ExpressionStmtNode {
                    position: self.current.position,
                    child: Box::new(eq_left),
                })),
            },

            Err(err) => Err(err),
        }
    }
}
