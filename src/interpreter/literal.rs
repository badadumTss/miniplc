use crate::core::errors::EvaluationError;

use crate::core::{ast::LiteralExprNode, variable::Variable};

use super::Interpreter;

impl Interpreter {
    pub(crate) fn eval_literal(
        &mut self,
        node: LiteralExprNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        Ok(node.value)
    }
}
