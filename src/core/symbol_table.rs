use std::{collections::HashMap, fmt::Display};

use crate::scanner::position::Position;

use super::types::Type;

#[derive(Clone, Copy, Debug)]
pub enum SymbolType {
    Function,
    Procedure,
    Var,
    VarParam,
    Param,
}

#[derive(Clone, Debug, Copy)]
pub struct Symbol {
    pub s_type: SymbolType,
    pub r_type: Type,
    pub position: Position,
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn get(&self, key: &str) -> Option<&Symbol> {
        self.symbols.get(key)
    }

    pub fn insert(&mut self, key: String, value: Symbol) -> Option<Symbol> {
        self.symbols.insert(key, value)
    }

    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
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
        for (name, sym) in self.symbols.iter() {
            res = format!("{}, ({}, {})", res, name, sym);
        }
        write!(f, "{}", res)
    }
}
