use std::fmt::Display;

use crate::scanner::position::Position;

use super::types::Type;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SymbolType {
    Function,
    Procedure,
    Var,
    VarParam,
    Param,
    Arr,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub s_type: SymbolType,
    pub r_type: Type,
    pub position: Position,
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn get(&self, key: String) -> Option<Symbol> {
        self.symbols
            .iter()
            .find(|sym| sym.name.eq_ignore_ascii_case(&key))
            .cloned()
    }

    pub fn push(&mut self, sym: Symbol) {
        self.symbols.push(sym)
    }

    pub fn pop(&mut self) -> Option<Symbol> {
        self.symbols.pop()
    }

    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.s_type, self.r_type)
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for sym in self.symbols.iter() {
            res = format!("{}, ({}, {})", res, sym.name, sym);
        }
        write!(f, "{}", res)
    }
}
