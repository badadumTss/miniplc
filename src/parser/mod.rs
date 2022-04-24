mod assert;
mod expressions;
mod for_loop;
mod print;
mod read;
mod statements;
mod function;
mod procedure;
mod block;
mod var_assignment;
mod var_declaration;
use std::collections::HashMap;

use log::trace;

use crate::core::ast::*;
use crate::core::errors::SyntaxError;
use crate::core::token::{Kind, Token};
use crate::core::variable::Type;
use crate::scanner::position::Position;
use crate::scanner::Scanner;

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
    previous: Token,
    current: Token,
    next: Option<Token>,
    // panic: bool,
    syntax_errors: Vec<SyntaxError>,
    context: HashMap<String, (Type, Position)>,
}

impl Parser {
    pub fn new(src: String) -> Parser {
        Parser {
            scanner: Scanner::new(src),
            previous: Token {
                kind: Kind::InitParser,
                lexeme: "".to_string(),
                position: Position::new(0, 0, 0),
            },
            current: Token {
                kind: Kind::InitParser,
                lexeme: "".to_string(),
                position: Position::new(0, 0, 0),
            },
            next: None,
            // panic: false,
            syntax_errors: vec![],
            context: HashMap::new(),
        }
    }

    /// Main parse function, gives the source to the scanner and
    /// iteratively looks at tokens, returning an AST (fake, since is
    /// practically a parse tree) that rapresents the program, the
    /// interpreter can then take the result and start interpreting it
    /// via the visitor pattern
    pub fn parse(&mut self) -> Result<ASTNode, Vec<SyntaxError>> {
        let mut program_name: Option<ProgramNameNode> = None;
        let mut procedures: Vec<ProcedureDeclNode> = Vec::new();
        let mut functions: Vec<FunctionDeclNode> = Vec::new();
        let mut main_block: Option<BlockNode> = None;
        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(ASTNode::ProgramName(node)) => program_name = Some(node),
                Ok(ASTNode::ProcedureDecl(node)) => procedures.push(node),
                Ok(ASTNode::FunctionDecl(node)) => functions.push(node),
                Ok(ASTNode::Block(node)) => main_block = Some(node),
                Ok(ASTNode::EofStmt(_)) => break,
                Ok(other) => self.syntax_errors.push(SyntaxError::new(
                    self.current.position,
                    self.scanner.curr_line(),
                    format!("Found an {} in global scope wich makes no sense", other),
                )),
                Err(errors) => {
                    for e in errors {
                        self.syntax_errors.push(e);
                    }
                    self.syncronize(); // goes to next statement
                }
            }
        }

        if self.syntax_errors.is_empty() {
            if let Some(name) = program_name {
                if let Some(main) = main_block {
                    Ok(ASTNode::Program(ProgramNode {
                        program_name: name,
                        functions: functions.into_boxed_slice(),
                        procedures: procedures.into_boxed_slice(),
                        main_block: main,
                    }))
                } else {
                    Err(vec![SyntaxError::new(
                        Position::new(0, 0, 0),
                        self.scanner.line(0),
                        "Main block is nowhere to be found".to_string(),
                    )])
                }
            } else {
                Err(vec![SyntaxError::new(
                    Position::new(0, 0, 0),
                    self.scanner.line(0),
                    "Found no program name in first line".to_string(),
                )])
            }
        } else {
            Err(self.syntax_errors.clone())
        }
    }

    /// Checks weather the current token kind matches the given one
    pub fn matches(&self, kind_to_match: Kind) -> bool {
        self.current.kind == kind_to_match
    }

    /// Returns weather the parsing process is finished, that is if
    /// the given source is all consumed
    pub fn is_at_end(&self) -> bool {
        self.matches(Kind::Eof)
    }

    /// Advances by one the tokens via the scanner function
    pub fn advance(&mut self) -> Token {
        trace!("{:?}", self.current);
        self.previous = self.current.clone();
        if let Some(reff) = self.next.clone() {
            self.current = reff;
            self.next = None;
            return self.current.clone();
        }
        while self.previous == self.current && !self.is_at_end() {
            match &self.scanner.next_token() {
                /* Doesn't stop until it finds a valid token, and
                 * next_token eventually returns a valid token (Eof as
                 * last thing) */
                Err(e) => self.error_at_current(e),
                Ok(token) => self.current = token.clone(),
            };
        }
        self.current.clone()
    }

    /// Restores the previous token as the current, but can be only done once
    pub fn go_back(&mut self) {
        self.next = Some(self.current.clone());
        self.current = self.previous.clone();
    }

    /// Utility function to push an error to the error stack
    pub fn error_at_current(&mut self, error: &SyntaxError) {
        self.syntax_errors.push(error.clone());
    }
}
