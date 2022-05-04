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
                let r_expr = self.parse_unary()?;
                match r_expr.r_type() {
                    Type::Simple(SimpleType::Bool) => Ok(ASTNode::UnaryExpression(UnaryExprNode {
                        position: bang.position,
                        operand: bang,
                        expression: Box::new(r_expr.clone()),
                        r_type: r_expr.r_type(),
                    })),
                    other => Err(vec![self.error_at_current(
                        format!("Expected expression of type bool, found {}", other).as_str(),
                    )]),
                }
            }
            Kind::Identifier => {
                trace!("found identifier: {}", self.current.clone().lexeme);
                match self.get_symbol(self.current.lexeme.clone()) {
                    Some(sym) => match sym.s_type {
                        SymbolType::Function => {
                            let to_return = self.parse_function_call()?;
                            self.advance();
                            Ok(to_return)
                        }
                        SymbolType::Procedure => Err(vec![
			    self.error_at_current(
				"Procedures produce no value to return, cannot be used inside an expression"
			    )]),
                        _ => {
                            trace!("found identifier {}", self.current.clone().lexeme);
                            let sym = self.parse_symbol()?;
                            self.advance();
                            Ok(sym)
                        }
                    },
                    None => Err(vec![self.error_at_current(
                        format!("Unknown symbol in expression: {}", self.current.lexeme).as_str(),
                    )]),
                }
            }
            Kind::True => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Bool(true),
                    r_type: Type::Simple(SimpleType::Bool),
                }))
            }
            Kind::False => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Bool(true),
                    r_type: Type::Simple(SimpleType::Bool),
                }))
            }
            Kind::LitString => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::String(self.previous.clone().lexeme),
                    r_type: Type::Simple(SimpleType::String),
                }))
            }
            Kind::LitInt => {
                self.advance();
                Ok(ASTNode::Literal(LiteralExprNode {
                    position: self.previous.position,
                    value: Object::Int(self.previous.clone().lexeme.parse::<i64>().unwrap()),
                    r_type: Type::Simple(SimpleType::Int),
                }))
            }
            Kind::LeftParen => match self.parse_expression() {
                Ok(node) => match self.current.kind {
                    Kind::RightParen => {
                        self.advance();
                        Ok(node)
                    }
                    _ => Err(vec![self.error_at_current("Missing closing bracket")]),
                },
                Err(synerr) => Err(synerr),
            },
            Kind::LeftSquare => self.parse_array(),

            other => {
                Err(vec![self.error_at_current(
                    format!("Unexpected token {:?}", other).as_str(),
                )])
            }
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse factor");
        let left_factor = self.parse_unary()?;
        match self.current.kind {
            Kind::Slash | Kind::Star => {
                let op = self.current.clone();
                let right_factor = self.parse_unary()?;
                if left_factor.r_type() == right_factor.r_type()
                    && left_factor.r_type() == Type::Simple(SimpleType::Int)
                {
                    return Ok(ASTNode::BinaryExpression(BinaryExprNode {
                        position: self.current.position,
                        left: Box::new(left_factor),
                        op: op.clone(),
                        op_type: match op.kind {
                            Kind::Slash => BinaryExprType::Division,
                            Kind::Star => BinaryExprType::Multiplication,
                            _ => panic!("Wtf? {:?}", op.kind),
                        },
                        right: Box::new(right_factor),
                        r_type: Type::Simple(SimpleType::Int),
                    }));
                }
            }
            _ => return Ok(left_factor),
        }
        Err(vec![
            self.error_at_current("Mismatching types in term parsing")
        ])
    }

    fn parse_term(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse term");
        let left_factor = self.parse_factor()?;
        match self.current.kind {
            Kind::Minus | Kind::Plus => {
                let op = self.current.clone();
                let right_factor = self.parse_factor()?;
                if left_factor.r_type() == right_factor.r_type() {
                    return Ok(ASTNode::BinaryExpression(BinaryExprNode {
                        position: self.current.position,
                        left: Box::new(left_factor.clone()),
                        op: op.clone(),
                        op_type: match op.kind {
                            Kind::Minus => BinaryExprType::Subtraction,
                            Kind::Plus => BinaryExprType::Addition,
                            _ => panic!("Wtf? {:?}", op.kind),
                        },
                        r_type: left_factor.r_type(),
                        right: Box::new(right_factor),
                    }));
                }
            }
            _ => return Ok(left_factor),
        }
        Err(vec![
            self.error_at_current("Mismatching types in term parsing")
        ])
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse comparison");
        let left_term = self.parse_term()?;
        match self.current.kind {
            Kind::Greater | Kind::GreaterEqual | Kind::Less | Kind::LessEqual => {
                let op = self.current.clone();
                let op_type = match op.kind {
                    Kind::Greater => BinaryExprType::LogicGreaterThan,
                    Kind::GreaterEqual => BinaryExprType::LogicGreaterThanEQ,
                    Kind::Less => BinaryExprType::LogicLessThan,
                    Kind::LessEqual => BinaryExprType::LogicLessThanEQ,
                    _ => panic!("Wtf? {:?}", op.kind), // seems risky
                                                       // but since
                                                       // the arm
                                                       // matched the
                                                       // kind must be
                                                       // one of these
                };
                let right_term = self.parse_term()?;
                Ok(ASTNode::BinaryExpression(BinaryExprNode {
                    position: self.current.position,
                    left: Box::new(left_term),
                    op,
                    op_type,
                    right: Box::new(right_term),
                    r_type: Type::Simple(SimpleType::Bool),
                }))
            }
            _ => Ok(left_term),
        }
    }

    fn parse_equality(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse equality");
        let comp_left = self.parse_comparison()?;
        if self.matches(Kind::Equal) {
            let equal = self.current.clone();
            let comp_right = self.parse_comparison()?;
            if comp_left.r_type() == comp_right.r_type()
                && comp_left.r_type() != Type::Simple(SimpleType::Void)
            {
                return Ok(ASTNode::BinaryExpression(BinaryExprNode {
                    position: self.current.position,
                    left: Box::new(comp_left),
                    op: equal,
                    op_type: BinaryExprType::LogicEQ,
                    right: Box::new(comp_right),
                    r_type: Type::Simple(SimpleType::Bool),
                }));
            }
        } else {
            return Ok(comp_left);
        }
        Err(vec![self.error_at_current(
            "mismatched types in equality expression",
        )])
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parse expression");
        let eq_left = self.parse_equality()?;

        if self.matches(Kind::And) {
            let and = self.current.clone();
            let eq_right = self.parse_equality()?;

            if eq_left.r_type() == eq_right.r_type()
                && eq_left.r_type() == Type::Simple(SimpleType::Bool)
            {
                return Ok(ASTNode::BinaryExpression(BinaryExprNode {
                    position: and.position,
                    left: Box::new(eq_left),
                    op: and,
                    op_type: BinaryExprType::LogicAND,
                    right: Box::new(eq_right),
                    r_type: Type::Simple(SimpleType::Bool),
                }));
            }

            Err(vec![self.error_at_current(
                format!(
                    "Expected type bool, found {:?}, {:?}",
                    eq_left.r_type(),
                    eq_right.r_type()
                )
                .as_str(),
            )])
        } else {
            Ok(eq_left)
        }
    }
}
