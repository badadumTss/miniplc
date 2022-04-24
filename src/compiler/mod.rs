mod source_control;
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
            ASTNode::Program(_) => todo!(),
            ASTNode::BinaryExpression(_) => todo!(),
            ASTNode::Identifier(_) => todo!(),
            ASTNode::Literal(_) => todo!(),
            ASTNode::UnaryExpression(_) => todo!(),
            ASTNode::VarReassignment(_) => todo!(),
            ASTNode::VariableDecl(_) => todo!(),
            ASTNode::ExpressionStmt(_) => todo!(),
            ASTNode::ForStmt(_) => todo!(),
            ASTNode::PrintStmt(_) => todo!(),
            ASTNode::ReadStmt(_) => todo!(),
            ASTNode::AssertStmt(_) => todo!(),
            ASTNode::EofStmt(_) => todo!(),
            ASTNode::FunctionDecl(_) => todo!(),
            ASTNode::ProcedureDecl(_) => todo!(),
            ASTNode::Block(_) => todo!(),
            ASTNode::ProgramName(_) => todo!(),
        }
    }

    pub fn compile(&mut self, source: String) -> Result<(), Vec<SyntaxError>> {
        let mut parser = Parser::new(source);
        match parser.parse() {
            Ok(ast) => Ok(self.compile_ast(ast)),
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
