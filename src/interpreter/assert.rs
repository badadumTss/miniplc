use crate::core::errors::EvaluationError;

use crate::core::ast::AssertStmtNode;
use crate::core::variable::Variable;

use super::Interpreter;

impl Interpreter {
    pub(crate) fn eval_assert(
        &mut self,
        node: AssertStmtNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match self.eval_expression_statement(node.expr) {
            Ok(variable) if variable.bool_value => Ok(variable),
            Ok(variable) => {
                println!("Assertion fail at {}", node.position);
                Ok(variable)
            }
            Err(parerr) => Err(parerr),
        }
    }
}
