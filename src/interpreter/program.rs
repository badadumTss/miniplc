use crate::core::ast::ASTNode;

use super::Interpreter;

impl Interpreter {
    /// Evaluates a given AST. For a rogram if some statement returns
    /// a ParseError (logical error) it simply breaks the loop and
    /// returns the error, does not actually keep going.
    pub fn eval(&mut self, stmt: ASTNode) {
        let mut errors = Vec::new();
        match stmt {
            ASTNode::Program(node) => {
                for stmt_prog in node.statements.iter() {
                    match stmt_prog {
                        ASTNode::EofStmt(_) => {}
                        other => match self.eval_statement(other.clone()) {
                            Ok(v) => {
                                if self.interactive {
                                    println!("return value: {}", v.string_value);
                                }
                            }
                            Err(error) => {
                                for e in error.iter() {
                                    errors.push(e.clone());
                                }
                                break;
                            }
                        },
                    }
                }
            }
            other => match self.eval_statement(other) {
                Ok(_) => {}
                Err(error) => {
                    for e in error.iter() {
                        errors.push(e.clone());
                    }
                }
            },
        }
        if !errors.is_empty() {
            for error in errors.iter() {
                println!("{}", error);
            }
        }
    }
}
