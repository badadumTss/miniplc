use crate::scanner::position::Position;
use colored::Colorize;
use std::fmt;

pub trait MiniPLError {
    fn get_error(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub position: Position,
    pub raw_line: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct EvaluationError {
    pub position: Position,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CompilationError {
    pub position: Position,
    pub description: String,
}

impl SyntaxError {
    pub fn new(position_: Position, raw_line_: String, description_: String) -> SyntaxError {
        SyntaxError {
            position: position_,
            raw_line: raw_line_,
            description: description_,
        }
    }
}

impl EvaluationError {
    pub fn new(position_: Position, description_: String) -> EvaluationError {
        EvaluationError {
            position: position_,
            description: description_,
        }
    }
}

impl MiniPLError for SyntaxError {
    fn get_error(&self) -> String {
        format! {"{} in {}:{} > {}\n\n\t{} |\t{}\n",
        "Syntax error".red().bold(),
        self.position.line,
        self.position.col,
        self.description.bold(),
        self.position.line,
        self.raw_line.bold()}
    }
}

impl MiniPLError for EvaluationError {
    fn get_error(&self) -> String {
        format! {
            "{} in {}:{} > {}\n",
            "Evaluation error".red().bold(),
            self.position.line,
            self.position.col,
            self.description.bold()
        }
    }
}

impl MiniPLError for CompilationError {
    fn get_error(&self) -> String {
        format! {
            "{} in {}:{} > {}\n\n\t{}",
            "Compilation error".red().bold(),
            self.position.line,
            self.position.col,
            self.description.bold(),
            self.position.line,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_error())
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_error())
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_error())
    }
}
