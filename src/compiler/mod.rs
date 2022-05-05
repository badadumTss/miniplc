mod block;
mod funcions;
mod procedures;
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
            ASTNode::ProgramName(node) => self.compile_program_name(node),
            ASTNode::FunctionDecl(f_node) => self.compile_function(f_node),
            ASTNode::ProcedureDecl(p_node) => self.compile_procedure(p_node),
            ASTNode::Block(b_node) => self.compile_block(b_node),
            ASTNode::BinaryExpression(exp_node) => self.compile_expression(expr_node),
            ASTNode::VarName(var_name) => self.compile_var_name(var_name),
            ASTNode::ArrayRef(a_ref_node) => self.compile_array_ref(a_ref_node),
            ASTNode::Literal(lit) => self.compile_lit(lit),
            ASTNode::UnaryExpression(expr) => self.compile_unary(expr),
            ASTNode::VarReassignment(ass) => self.compile_var_assignment(ass),
            ASTNode::VariableDecl(decl) => self.compile_var_decl(decl),
            ASTNode::WhileStmt(while_stmt) => self.compile_while(while_stmt),
            ASTNode::IfStmt(if_stmt) => self.compile_if(if_stmt),
            ASTNode::ElseStmt(else_stmt) => self.compile_else(else_stmt),
            ASTNode::PrintStmt(prnt) => self.compile_print(prnt),
            ASTNode::ReadStmt(read) => self.compile_read(read),
            ASTNode::AssertStmt(assert) => self.compile_assert(assert),
            ASTNode::FunctionCallStmt(fn_call) => self.compile_function_call(fn_call),
            ASTNode::ProcedureCallStmt(proc_call) => self.compile_procedure_call(proc_call),
            ASTNode::ReturnStmt(ret) => self.compile_return(ret),
            ASTNode::EofStmt(eof) => {}
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
