use log::debug;

use crate::core::errors::EvaluationError;

use crate::core::{ast::PrintStmtNode, variable::Variable};

use super::Interpreter;

impl Interpreter {
    pub(crate) fn eval_print(
        &mut self,
        node: PrintStmtNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        debug!("print invoked");
        match self.eval_expression_statement(node.to_print) {
            Ok(to_print) => {
                print!("{}", to_print.string_value);
                Ok(to_print)
            }
            Err(e) => Err(e),
        }
    }
}
