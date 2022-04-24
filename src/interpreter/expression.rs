use log::debug;

use crate::core::ast::BinaryExprType;
use crate::core::errors::EvaluationError;

use crate::core::token::Kind;
use crate::core::variable::Type;
use crate::core::{
    ast::{ASTNode, BinaryExprNode, ExpressionStmtNode, UnaryExprNode},
    variable::Variable,
};

use super::Interpreter;

impl Interpreter {
    /// Evaluates a binary expression, checks weather both operands
    /// have the same type and then executes it
    pub(crate) fn eval_expression_binary(
        &mut self,
        node: BinaryExprNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        let left_arm = self.eval_statement(node.left.as_ref().clone())?;
        let right_arm = self.eval_statement(node.right.as_ref().clone())?;
        let ltype = left_arm.clone().actual_type;
        let rtype = right_arm.clone().actual_type;

        // Type check
        if ltype != rtype {
            return Err(vec![EvaluationError {
                position: node.op.position,
                description: format!(
                    "Left arm and right arm of expression have different types: {} and {}",
                    ltype, rtype
                ),
            }]);
        }

        debug!(
            "Left-arm: {}\tright-arm: {}\ttypes: {}, {}\toperation: {}",
            left_arm.int_value, right_arm.int_value, ltype, rtype, node.op_type
        );

        // They have both the same type, the operation is possible
        match node.op_type {
            BinaryExprType::Addition if ltype == Type::Integer => {
                Ok(Variable::from_int(left_arm.int_value + right_arm.int_value))
            }
            BinaryExprType::Subtraction if ltype == Type::Integer => {
                Ok(Variable::from_int(left_arm.int_value - right_arm.int_value))
            }
            BinaryExprType::Multiplication if ltype == Type::Integer => {
                Ok(Variable::from_int(left_arm.int_value * right_arm.int_value))
            }
            BinaryExprType::Division if ltype == Type::Integer => {
                Ok(Variable::from_int(left_arm.int_value / right_arm.int_value))
            }
            BinaryExprType::LogicEQ if ltype == Type::Integer => Ok(Variable::from_bool(
                left_arm.int_value == right_arm.int_value,
            )),
            BinaryExprType::LogicGreaterThan if ltype == Type::Integer => Ok(Variable::from_bool(
                left_arm.int_value > right_arm.int_value,
            )),
            BinaryExprType::LogicGreaterThanEQ if ltype == Type::Integer => Ok(
                Variable::from_bool(left_arm.int_value >= right_arm.int_value),
            ),
            BinaryExprType::LogicLessThan if ltype == Type::Integer => Ok(Variable::from_bool(
                left_arm.int_value < right_arm.int_value,
            )),
            BinaryExprType::LogicLessThanEQ if ltype == Type::Integer => Ok(Variable::from_bool(
                left_arm.int_value <= right_arm.int_value,
            )),
            BinaryExprType::Addition if ltype == Type::String => Ok(Variable::from_string(
                left_arm.string_value + &right_arm.string_value,
            )),
            BinaryExprType::LogicAND if ltype == Type::Boolean => Ok(Variable::from_bool(
                left_arm.bool_value && right_arm.bool_value,
            )),
            other => Err(vec![EvaluationError {
                position: node.position,
                description: format!(
                    "Incompatible operation {} for type {} and {}",
                    other, ltype, rtype
                ),
            }]),
        }
    }

    pub(crate) fn eval_expression_unary(
        &mut self,
        node: UnaryExprNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match node.operand.kind {
            Kind::Bang => match self.eval_statement(node.expression.as_ref().clone()) {
                Ok(var) => match var.actual_type {
                    Type::Boolean => Ok(Variable::from_bool(!var.bool_value)),
                    other => Err(vec![EvaluationError {
                        position: node.position,
                        description: format!("Expected boolean expression, found {}", other),
                    }]),
                },
                Err(err) => Err(err),
            },
            other => Err(vec![EvaluationError {
                position: node.position,
                description: format!("Unknown operation {}", other),
            }]),
        }
    }

    pub(crate) fn eval_expression_statement(
        &mut self,
        node: ExpressionStmtNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match &node.child.as_ref() {
            ASTNode::BinaryExpression(node) => self.eval_expression_binary(node.clone()),
            ASTNode::UnaryExpression(node) => self.eval_expression_unary(node.clone()),
            ASTNode::ExpressionStmt(node) => self.eval_expression_statement(node.clone()),
            ASTNode::Literal(node) => self.eval_literal(node.clone()),
            ASTNode::Identifier(node) => self.eval_identifier(node.clone()),
            other => Err(vec![EvaluationError {
                position: node.position,
                description: format!("Expected expression, found {}", other),
            }]),
        }
    }
}
