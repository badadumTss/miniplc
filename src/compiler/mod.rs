mod array;
mod assert;
mod block;
mod expression;
mod funcions;
mod if_stmt;
mod print;
mod procedures;
mod program;
mod program_name;
mod read;
mod return_stmt;
mod var;
mod while_stmt;

use crate::{
    core::{
        ast::ASTNode,
        errors::{CompilationError, SyntaxError},
    },
    parser::Parser,
};
use std::{
    borrow::Borrow,
    fs::File,
    io::{Error, Read},
};

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
    source: String,
    raw_instructions: String,
    pub scope: String,
    pub label: usize,
}

impl Compiler {
    pub fn insert_header(&mut self) {
        self.source = "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <stdbool.h>\nint main(){\n"
            .to_string()
            + &self.source;
    }

    pub fn insert_footer(&mut self) {
        self.source += "\n}";
    }

    pub fn push_instruction(&mut self, instr: String) {
        self.raw_instructions = format!("{}{}\n", self.raw_instructions, instr);
    }

    pub fn push_instruction_raw(&mut self, instr: String) {
        self.raw_instructions = format!("{}{}", self.raw_instructions, instr);
    }

    pub fn gen_source(&mut self) {
        self.source = String::new();
        self.insert_header();
        self.source += self.raw_instructions.borrow();
        self.insert_footer();
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn new() -> Compiler {
        Compiler {
            c_errors: vec![],
            s_errors: vec![],
            state: State::Sane,
            source: String::new(),
            raw_instructions: String::new(),
            scope: "main".to_string(),
            label: 0,
        }
    }

    pub fn advance_label(&mut self) -> usize {
        let to_ret = self.label;
        self.label += 1;
        to_ret
    }

    pub fn compile_ast(&mut self, ast: ASTNode) {
        match ast {
            ASTNode::Program(p) => {
                self.compile_program(p);
                self.gen_source();
                println!("{}", self.get_source())
            }
            ASTNode::ProgramName(node) => self.compile_program_name(node),
            ASTNode::FunctionDecl(f_node) => self.compile_function(f_node),
            ASTNode::ProcedureDecl(p_node) => self.compile_procedure(p_node),
            ASTNode::Block(b_node) => self.compile_block(b_node),
            ASTNode::BinaryExpression(exp_node) => self.compile_expression(exp_node),
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
            ASTNode::EofStmt(_) => {}
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
                self.gen_source();
                Ok(self.get_source())
            }
            Err(mut errs) => {
                self.s_errors.append(&mut errs);
                self.state = State::Error;
                Ok("".to_string())
            }
        }
    }

    pub fn push_c_error(&mut self, node: ASTNode, msg: &str) {
        self.state = State::Error;
        self.c_errors.push(CompilationError {
            description: msg.to_string(),
            position: node.position(),
        });
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
