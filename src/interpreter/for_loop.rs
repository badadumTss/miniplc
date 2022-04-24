use log::debug;

use crate::core::ast::ASTNode;
use crate::core::errors::EvaluationError;

use crate::core::variable::Type;
use crate::core::{ast::ForStmtNode, variable::Variable};
use crate::scanner::position::Position;

use super::Interpreter;

impl Interpreter {
    fn error_child_changes_var(&self, pos: Position, lexeme: String) -> EvaluationError {
        EvaluationError::new(
            pos,
            format!(
                "Statements inside for loop are not allowed to edit increment variable ({}) value",
                lexeme
            ),
        )
    }

    fn check_stmts(
        &self,
        stmts: &[ASTNode],
        incr_lexeme: String,
        errors: &mut Vec<EvaluationError>,
    ) {
        for statement in stmts.iter() {
            // check weather child statements do not change increment variable
            match statement {
                ASTNode::VarReassignment(child) => {
                    if child.variable_to_reassign.lexeme == incr_lexeme {
                        errors.push(
                            self.error_child_changes_var(child.position, incr_lexeme.clone()),
                        );
                    }
                }
                ASTNode::ReadStmt(child) => {
                    if child.variable_to_read_in.lexeme == incr_lexeme {
                        errors.push(
                            self.error_child_changes_var(child.position, incr_lexeme.clone()),
                        );
                    }
                }
                ASTNode::VariableDecl(child) => {
                    if child.id.lexeme == incr_lexeme {
                        errors.push(
                            self.error_child_changes_var(child.position, incr_lexeme.clone()),
                        );
                    }
                }
                ASTNode::ForStmt(child) => {
                    if child.increment.lexeme == incr_lexeme.clone() {
                        errors.push(
                            self.error_child_changes_var(child.position, incr_lexeme.clone()),
                        );
                    }
                    // check nested for loops for statements that try to modify the increment variable
                    self.check_stmts(&child.statements.clone(), incr_lexeme.clone(), errors);
                }
                _ => {}
            }
        }
    }

    pub(crate) fn eval_for(&mut self, node: ForStmtNode) -> Result<Variable, Vec<EvaluationError>> {
        let mut errors: Vec<EvaluationError> = Vec::new();
        let from_stmt = self.eval_expression_statement(*(node.clone().range_start));
        let to_stmt = self.eval_expression_statement(*(node.clone().range_end));
        let increment_var = self.context.get(&node.increment.lexeme);

        // Checking types and validity of everything before evaluating
        match from_stmt.clone() {
            Ok(from_var) => match from_var.actual_type {
                Type::Integer => {}
                other => errors.push(EvaluationError::new(
                    node.clone().range_start.position,
                    format!("Unexpected range start type {}", other),
                )),
            },
            Err(e) => {
                for err in e {
                    errors.push(err.clone());
                }
            }
        }

        match to_stmt.clone() {
            Ok(to_var) => match to_var.actual_type {
                Type::Integer => {}
                other => errors.push(EvaluationError::new(
                    node.clone().range_end.position,
                    format!("Unexpected range end type {}", other),
                )),
            },
            Err(e) => {
                for err in e {
                    errors.push(err.clone());
                }
            }
        }

        match increment_var {
            Some(v) => match &v.actual_type {
                Type::Integer => {
                    self.check_stmts(
                        &node.statements.clone(),
                        node.clone().increment.lexeme,
                        &mut errors,
                    );
                }
                other => errors.push(EvaluationError::new(
                    node.increment.position,
                    format!(
                        "Increment variable {} must be of type Integer, type {} found instead",
                        node.increment.lexeme, other
                    ),
                )),
            },
            None => {
                errors.push(EvaluationError::new(
                    node.position,
                    format!(
                        "Increment variable {} must be initialized before looping over it",
                        node.clone().increment.lexeme
                    ),
                ));
            }
        }

        // If there are errors the function just returns
        if !errors.is_empty() {
            return Err(errors);
        }

        // Reaching this means everything is ok and can be unwrapped safely
        let from = from_stmt.unwrap();
        let to = to_stmt.unwrap();

        debug!("from: {}\tto: {}", from.int_value, to.int_value);

        // actual for loop
        for i in from.int_value..(to.int_value + 1) {
            self.context
                .insert(node.clone().increment.lexeme, Variable::from_int(i));
            debug!("i: {}", i);
            for stmt in node.statements.iter() {
                debug!("Statement: {}", stmt.clone());
                self.eval(stmt.clone());
            }
        }

        let final_val = Variable::from_int(to.int_value + 1);

        self.context
            .insert(node.clone().increment.lexeme, final_val.clone());

        Ok(final_val)
    }
}
