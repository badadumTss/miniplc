use crate::core::errors::EvaluationError;

use crate::core::{ast::IdentifierExprNode, variable::Variable};

use super::Interpreter;

impl Interpreter {
    pub(crate) fn eval_identifier(
        &mut self,
        node: IdentifierExprNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match self.context.get(&node.id.lexeme) {
            Some(var) => Ok(var.clone()),
            None => Err(vec![EvaluationError {
                position: node.position,
                description: format!("Uninitialized variable {}", node.id.lexeme),
            }]),
        }
    }
}
