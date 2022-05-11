use log::trace;

use crate::{
    advance_with_expected,
    core::{
        ast::{ASTNode, ProcedureCallNode, ProcedureDeclNode},
        errors::SyntaxError,
        symbol_table::{Symbol, SymbolType},
        token::Kind,
        types::{SimpleType, Type},
    },
    current_with_expected,
};

use super::Parser;

impl Parser {
    pub fn parse_procedure(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing procedure declaration");
        advance_with_expected!(Kind::Identifier, self, {
            let id = self.current.clone();
            advance_with_expected!(Kind::LeftParen, self, {
                let args = self.parse_parameters()?;
                current_with_expected!(
                    Kind::RightParen,
                    self,
                    advance_with_expected!(
                        Kind::Semicolon,
                        self,
                        advance_with_expected!(Kind::Begin, self, {
                            trace!("procedure {}, args: {}", id.lexeme, args);
                            let mut ctx = self.context.pop().unwrap();
                            ctx.push(Symbol {
                                name: id.lexeme.clone(),
                                s_type: SymbolType::Procedure,
                                r_type: Type::Simple(SimpleType::Void),
                                position: id.position,
                                args: Some(Box::new(args.clone())),
                            });
                            self.context.push(ctx);
                            self.context.push(args.clone());
                            let block = self.parse_block()?;
                            trace!("block parsed");
                            Ok(ASTNode::ProcedureDecl(ProcedureDeclNode {
                                position: self.current.position,
                                name: id.lexeme,
                                args,
                                block: Box::new(block),
                            }))
                        })
                    )
                )
            })
        })
    }

    pub fn parse_procedure_call(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        trace!("Parsing procedure call");
        let f_name = self.current.clone();
        let f_sym_opt = self.get_symbol(f_name.lexeme.clone());
        if let Some(f_sym) = f_sym_opt {
            advance_with_expected!(Kind::LeftParen, self, {
                let params = self.parse_call_parameters()?;
                if let Some(args) = f_sym.args {
                    let args_ref = args.as_ref();
                    let same_len = params.len() == args_ref.len();
                    if same_len {
                        let mut args_with_lexeme: Vec<(String, ASTNode)> = Vec::new();
                        for (a, b) in args_ref.iter().zip(params.iter()) {
                            if a.r_type != b.r_type() {
                                return Err(vec![
                                    self.error_at_current("Mismatching types in procedure call")
                                ]);
                            }
                            args_with_lexeme.push((a.clone().name, b.clone()));
                        }
                        current_with_expected!(
                            Kind::RightParen,
                            self,
                            advance_with_expected!(
                                Kind::Semicolon,
                                self,
                                Ok(ASTNode::ProcedureCallStmt(ProcedureCallNode {
                                    position: f_name.position,
                                    args: args_with_lexeme.into_boxed_slice(),
                                    target: f_name.lexeme,
                                }))
                            )
                        )
                    } else {
                        Err(vec![self.error_at_current(
                            "Function call with wrong number of parameters",
                        )])
                    }
                } else {
                    Err(vec![self.error_at_current(
                        "Initialized function with empty arg table",
                    )])
                }
            })
        } else {
            Err(vec![self.error_at_current(
                format!("Unknown procedure {}", f_name.lexeme).as_str(),
            )])
        }
    }
}
