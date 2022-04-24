use std::io::{self, Write};

use crate::core::ast::ASTNode;
use crate::core::errors::EvaluationError;
use crate::core::variable::Type;
use crate::core::{ast::ReadStmtNode, variable::Variable};
use crate::parser::Parser;

use super::Interpreter;

impl Interpreter {
    /// Read statement evaluation function. Reads directly from the
    /// standard input. Unfortunately for this charatersitic im unable
    /// for now to test it
    #[cfg(not(tarpaulin_include))]
    pub(crate) fn eval_read(
        &mut self,
        node: ReadStmtNode,
    ) -> Result<Variable, Vec<EvaluationError>> {
        let mut buf = String::new();
        if let Some(var) = self.context.get(&node.variable_to_read_in.lexeme) {
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();

            // prepending double quotes if the actual type is a
            // string, so that in that case it is parsed as a literal
            // string
            match Parser::new(match var.actual_type {
                Type::String => format!("\"{}\"", buf),
                _ => buf,
            })
            .parse_unary()
            .unwrap()
            {
                ASTNode::Literal(mut ast_node) => {
                    match var.actual_type {
                        Type::Boolean => ast_node.value.make_bool(),
                        Type::Integer => ast_node.value.make_int(),
                        Type::String => ast_node.value.make_string(),
                    }
                    self.context
                        .insert(node.variable_to_read_in.lexeme, ast_node.clone().value);
                    Ok(ast_node.value)
                }
                _ => Err(vec![EvaluationError {
                    position: node.position,
                    description: format!(
                        "Unable to assign this value to the variable {}",
                        node.variable_to_read_in.lexeme
                    ),
                }]),
            }
        } else {
            Err(vec![EvaluationError {
                position: node.position,
                description: "Trying to read into an uninitialized variable".to_string(),
            }])
        }
    }
}
