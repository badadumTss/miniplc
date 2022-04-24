use log::debug;

use crate::core::errors::EvaluationError;

use crate::core::variable::Type;
use crate::core::{
    ast::{VarReassignmentExprNode, VariableDeclNode},
    variable::Variable,
};

use super::Interpreter;

impl Interpreter {
    pub(crate) fn eval_var_reassignment(
        &mut self,
        node: VarReassignmentExprNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        match self.eval_expression_statement(node.new_value) {
            Ok(new_val) => match self
                .context
                .insert(node.variable_to_reassign.clone().lexeme, new_val.clone())
            {
                Some(old_val) => {
                    if old_val.actual_type == new_val.actual_type {
                        Ok(new_val)
                    } else {
                        Err(vec![EvaluationError {
                            position: node.position,
                            description: format!(
				"Mismatching types in variable reassignment: found {} instead of {}",
				new_val.actual_type, old_val.actual_type
			    ),
                        }])
                    }
                }
                None => Err(vec![EvaluationError {
                    position: node.position,
                    description: format!(
                        "Trying to reassign an uninitialized variable: {}",
                        node.variable_to_reassign.lexeme
                    ),
                }]),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) fn eval_var_declaration(
        &mut self,
        node: VariableDeclNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        let mut should_break = None;

        let value = match node.value {
            Some(val) => match self.eval_expression_statement(val) {
                Ok(var) if var.actual_type == node.var_type => var,
                Ok(var) => {
                    should_break = Some(vec![EvaluationError {
                        position: node.position,
                        description: format!(
                            "Mismatched type: cannot assign {} to a {} variable",
                            var.actual_type, node.var_type
                        ),
                    }]);
                    var
                }
                Err(error) => {
                    should_break = Some(error);
                    Variable::from_int(0)
                }
            },
            None => match node.var_type {
                Type::Boolean => Variable::from_bool(false),
                Type::Integer => Variable::from_int(0),
                Type::String => Variable::from_string(String::new()),
            },
        };

        if let Some(e) = should_break {
            Err(e)
        } else if self.context.contains_key(&node.id.lexeme) {
            Err(vec![EvaluationError {
                position: node.position,
                description: format!(
                    "Trying to initialize an already initialized variable: {}",
                    node.id.lexeme
                ),
            }])
        } else {
            self.context.insert(node.id.lexeme.clone(), value.clone());
            debug!(
                "new {} value: {}",
                node.id.lexeme,
                self.context.get(&node.id.lexeme).unwrap().string_value
            );
            Ok(value)
        }
    }
}
