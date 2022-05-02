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
                            self.context.push(args.clone());
                            let block = self.parse_block()?;
                            trace!("block parsed");
                            match self.context.pop() {
                                Some(mut ctx) => {
                                    ctx.insert(
                                        id.lexeme.clone(),
                                        Symbol {
                                            s_type: SymbolType::Function,
                                            r_type: Type::Simple(SimpleType::Void),
                                            position: id.position,
                                        },
                                    );
                                    self.context.push(ctx);
                                }
                                None => {
                                    panic!("Dropped global context while parsing")
                                }
                            };
                            Ok(ASTNode::ProcedureDecl(ProcedureDeclNode {
                                position: self.current.position,
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
        advance_with_expected!(Kind::LeftParen, self, {
            let params = self.parse_call_parameters()?;
            current_with_expected!(
                Kind::RightParen,
                self,
                Ok(ASTNode::ProcedureCallStmt(ProcedureCallNode {
                    position: f_name.position,
                    args: params,
                    target: f_name.lexeme,
                }))
            )
        })
    }
}
