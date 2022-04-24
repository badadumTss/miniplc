mod assert;
mod expression;
mod for_loop;
mod identifier;
mod literal;
mod print;
mod program;
mod read;
mod statements;
mod var;

use crate::core::variable::Variable;
use crate::parser::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Read;
use std::io::Write;

/// Actual interpreter, takes the ASTNode rapresenting the program and
/// parses it, reflecting the changes inside of the context, a map
/// containing the active variables
pub struct Interpreter {
    context: HashMap<String, Variable>,
    interactive: bool,
}

impl Interpreter {
    pub fn new(interactive: bool) -> Interpreter {
        Interpreter {
            context: HashMap::new(),
            interactive,
        }
    }

    pub fn run(&mut self, source: String) {
        let mut parser = Parser::new(source);
        match parser.parse() {
            Ok(ast) => self.eval(ast),
            Err(vec) => {
                for el in vec {
                    println!("{}", el);
                }
            }
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn run_file(&mut self, file_name: String) -> Result<(), Error> {
        let mut file = File::open(file_name)?;
        let mut source = String::new();

        file.read_to_string(&mut source)?;
        self.run(source);
        Ok(())
    }

    #[cfg(not(tarpaulin_include))]
    pub fn shell(&mut self) -> Result<(), Error> {
        let mut stmt = String::new();
        let stdin = io::stdin();
        loop {
            stmt.clear();
            print!("minipli> ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut stmt)?;
            if !stmt.chars().nth(stmt.len() - 2).eq(&Some(';')) {
                stmt.push(';');
            }
            self.run(stmt.clone());
        }
    }
}
