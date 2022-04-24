use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub char_number: usize,
    pub line: i64,
    pub col: i64,
}

impl Position {
    pub fn new(cn: usize, l: i64, c: i64) -> Position {
        Position {
            char_number: cn,
            line: l,
            col: c,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
