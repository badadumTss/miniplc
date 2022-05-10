use crate::core::{
    ast::{ASTNode, BinaryExprNode, BinaryExprType, LiteralExprNode, UnaryExprNode},
    types::{SimpleType, Type},
};

use super::Compiler;

impl Compiler {
    pub fn compile_expression(&mut self, expr: BinaryExprNode) {
        let label = self.advance_label();
        self.push_instruction(format!(
            "{} left_arm_{};",
            expr.left.as_ref().clone().r_type().to_c_type(),
            label
        ));
        self.push_instruction(format!(
            "{} right_arm_{};",
            expr.right.as_ref().clone().r_type().to_c_type(),
            label
        ));

        self.compile_ast(expr.left.as_ref().clone());
        self.push_instruction(format!(
            "left_arm_{} = last_{};",
            label,
            expr.left.as_ref().clone().r_type().to_c_type()
        ));

        self.compile_ast(expr.right.as_ref().clone());
        self.push_instruction(format!(
            "right_arm_{} = last_{};",
            label,
            expr.right.as_ref().clone().r_type().to_c_type()
        ));
        match expr.clone().r_type {
            Type::Simple(s) => match s {
                SimpleType::Int => match expr.op_type {
                    BinaryExprType::Addition
                    | BinaryExprType::Subtraction
                    | BinaryExprType::Multiplication
                    | BinaryExprType::Division => self.push_instruction(format!(
                        "last_{} = right_arm_{} {} left_arm_{};",
                        expr.r_type.to_c_type(),
                        label,
                        expr.op.lexeme,
                        label
                    )),
                    _ => self.push_c_error(
                        ASTNode::BinaryExpression(expr.clone()),
                        format!("Valid int operations are +,-,*,/, not {}", expr.op.lexeme)
                            .as_str(),
                    ),
                },
                SimpleType::String => match expr.op_type {
                    BinaryExprType::Addition => self.push_instruction(format!(
                        "strcat(right_arm_{}, left_arm_{});",
                        label, label
                    )),
                    _ => self.push_c_error(
                        ASTNode::BinaryExpression(expr.clone()),
                        format!(
                            "allowed operations here are: (+), {} found instead",
                            expr.op.lexeme
                        )
                        .as_str(),
                    ),
                },
                SimpleType::Bool => match expr.op_type {
                    BinaryExprType::LogicGreaterThan
                    | BinaryExprType::LogicGreaterThanEQ
                    | BinaryExprType::LogicLessThan
                    | BinaryExprType::LogicLessThanEQ => self.push_instruction(format!(
                        "last_bool = left_arm_{} {} right_arm_{};",
                        label, expr.op.lexeme, label
                    )),
                    BinaryExprType::LogicEQ => self.push_instruction(format!(
                        "last_bool = left_arm_{} == right_arm_{};",
                        label, label
                    )),
                    BinaryExprType::LogicAND => self.push_instruction(format!(
                        "last_bool = left_arm_{} && right_arm_{};",
                        label, label
                    )),
                    _ => self.push_c_error(
                        ASTNode::BinaryExpression(expr.clone()),
                        format!(
			    "allowed operations here are: (and, or, >, <, >=, <=, =), {} found instead",
			    expr.op.lexeme
			)
                        .as_str(),
                    ),
                },
                SimpleType::Void => self.push_c_error(
                    ASTNode::BinaryExpression(expr),
                    "Binary expressions between void oprands are not allowed",
                ),
            },
            Type::Array(_) => self.push_c_error(
                ASTNode::BinaryExpression(expr),
                "Binary expressions between arrays are not allowed",
            ),
        }
    }

    pub fn compile_lit(&mut self, expr: LiteralExprNode) {
        match expr.r_type {
            Type::Simple(s) => match s {
                SimpleType::Int | SimpleType::Bool => self.push_instruction(format!(
                    "last_{} = {};",
                    expr.r_type.to_c_type(),
                    expr.value.to_c_lit()
                )),
                SimpleType::String => {
                    self.push_instruction("last_str = malloc(128 * sizeof(char));".to_string());
                    self.push_instruction(format!("strcpy(last_str, {});", expr.to_c_lit()));
                }
                SimpleType::Void => {
                    self.push_c_error(ASTNode::Literal(expr), "literal of type void?")
                }
            },
            Type::Array(a) => match a {
                SimpleType::Int => {
                    let label = self.advance_label();
                    self.push_instruction(format!("int* tmp_{}[] = {};", label, expr.to_c_lit()));
                    self.push_instruction(format!("last_int_arr = tmp_{};", label));
                }
                SimpleType::String => {
                    let label = self.advance_label();
                    self.push_instruction(format!("char** tmp_{}[] = {};", label, expr.to_c_lit()));
                    self.push_instruction(format!("last_str_arr = tmp_{};", label));
                }
                SimpleType::Bool => {
                    let label = self.advance_label();
                    self.push_instruction(format!("int* tmp_{}[] = {};", label, expr.to_c_lit()));
                    self.push_instruction(format!("last_bool_arr = tmp_{};", label));
                }
                SimpleType::Void => {
                    self.push_c_error(ASTNode::Literal(expr), "Found array of void expressions")
                }
            },
        }
    }

    pub fn compile_unary(&mut self, expr: UnaryExprNode) {
        self.compile_ast(expr.expression.as_ref().clone());
        self.push_instruction("last_bool = !last_bool;".to_string());
    }
}
