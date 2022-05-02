mod program;
mod source_control;
use log::error;

use crate::{
    core::{
        ast::ASTNode,
        errors::{CompilationError, SyntaxError},
    },
    parser::Parser,
};
use std::{
    fs::File,
    io::{Error, Read},
};

use self::source_control::SourceControl;

#[derive(Debug)]
pub enum State {
    Sane,
    Error,
}

#[derive(Debug)]
pub struct Compiler {
    pub c_errors: Vec<CompilationError>,
    pub s_errors: Vec<SyntaxError>,
    pub state: State,
    pub src_ctrl: SourceControl,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            c_errors: vec![],
            s_errors: vec![],
            state: State::Sane,
            src_ctrl: SourceControl::new(),
        }
    }

    pub fn compile_ast(&mut self, ast: ASTNode) {
        match ast {
            ASTNode::Program(p) => {
                self.compile_program(p);
                self.src_ctrl.gen_source();
                println!("{}", self.src_ctrl.get_source())
            }
            other => error!("Recived an unknown AST as compilation input"),
        }
    }

    pub fn compile(&mut self, source: String) -> Result<(), Vec<SyntaxError>> {
        let mut parser = Parser::new(source);
        match parser.parse() {
            Ok(ast) => {
                self.compile_ast(ast);
                Ok(())
            }
            Err(errs) => Err(errs),
        }
    }

    pub fn compile_file(&mut self, file_name: String) -> Result<String, Error> {
        let mut file = File::open(file_name)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;

        match self.compile(source) {
            Ok(_) => {
                self.src_ctrl.gen_source();
                Ok(self.src_ctrl.get_source())
            }
            Err(mut errs) => {
                self.s_errors.append(&mut errs);
                self.state = State::Error;
                Ok("".to_string())
            }
        }
    }
}