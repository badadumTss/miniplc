use log::trace;

use crate::{
    advance_with_expected,
    core::{
        ast::{ASTNode, ArrayRefExpr, VarNameNode, VarReassignmentExprNode},
        errors::SyntaxError,
        symbol_table::SymbolType,
        token::Kind,
        types::Type,
    },
    current_with_expected,
    parser::Parser,
};

impl Parser {
    /// Parser function for a variable assignment, reutrns an ASTNode
    /// that rapresents this statement
    pub fn parse_var_assignment(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("parsing var assignment");
        let id = self.current.clone(); // the current token is the identifier of the variable
        match self.get_symbol(id.lexeme.clone()) {
            Some(sym) if sym.s_type == SymbolType::Arr => {
                let nested_type = match sym.r_type {
                    Type::Simple(_) => panic!("Mismatching symbol type and effective type"),
                    Type::Array(s) => s,
                };
                match self.advance().kind {
                    Kind::LeftSquare => {
                        let pos = self.parse_unary()?;
                        current_with_expected!(
                            Kind::RightSquare,
                            self,
                            advance_with_expected!(Kind::ColonEqual, self, {
                                let new_val = self.parse_expression()?;
                                Ok(ASTNode::VarReassignment(VarReassignmentExprNode {
                                    position: id.position,
                                    variable_to_reassign: Box::new(ASTNode::ArrayRef(
                                        ArrayRefExpr {
                                            position: id.position,
                                            array: id,
                                            index: Box::new(pos),
                                            r_type: sym.r_type,
                                        },
                                    )),
                                    new_value: Box::new(new_val),
                                }))
                            })
                        )
                    }
                    Kind::ColonEqual => {
                        self.advance();
                        let new_arr = self.parse_array()?;
                        if new_arr.r_type().internal() != nested_type {
                            Err(vec![
                                self.error_at_current("mismatching types in array assignment")
                            ])
                        } else {
                            self.advance();
                            Ok(ASTNode::VarReassignment(VarReassignmentExprNode {
                                position: id.position,
                                variable_to_reassign: Box::new(ASTNode::VarName(VarNameNode {
                                    position: id.position,
                                    id,
                                    r_type: sym.r_type,
                                    s_type: sym.s_type,
                                })),
                                new_value: Box::new(new_arr),
                            }))
                        }
                    }
                    other => Err(vec![self.error_at_current(
                        format!(
                            "Expected either array referenceing or array redeclaration, found: {}",
                            other
                        )
                        .as_str(),
                    )]),
                }
            }
            Some(sym)
                if sym.s_type == SymbolType::Function || sym.s_type == SymbolType::Procedure =>
            {
                Err(vec![self.error_at_current(
                    "Symbol alread exists and is associated either to a funciton or to a procedure",
                )])
            }
            Some(sym) => {
                trace!("symbol found: {}", sym);
                advance_with_expected!(Kind::ColonEqual, self, {
                    let new_val = self.parse_expression()?;
                    if new_val.r_type() == sym.r_type {
                        Ok(ASTNode::VarReassignment(VarReassignmentExprNode {
                            position: id.position,
                            variable_to_reassign: Box::new(ASTNode::VarName(VarNameNode {
                                position: id.position,
                                id,
                                r_type: sym.r_type,
                                s_type: sym.s_type,
                            })),
                            new_value: Box::new(new_val),
                        }))
                    } else {
                        Err(vec![
                            self.error_at_current("Mismatching types in var assignment")
                        ])
                    }
                })
            }
            None => Err(vec![self.error_at_current(
                format!("Use of undeclared variable {}", id.lexeme).as_str(),
            )]),
        }
    }
}
