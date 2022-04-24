use crate::scanner::position::Position;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    // Symbols
    LeftParen,   // (
    RightParen,  // )
    LeftSquare,  // [
    RightSquare, // ]
    Minus,       // -
    Plus,        // +
    Semicolon,   // ;
    Slash,       // /
    Star,        // *
    Bang,        // !
    And,         // &

    // One or two character tokens.
    Equal,        // =
    Colon,        // :
    ColonEqual,   // :=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    Ddot,         // ..

    // Literals.
    Identifier, // Var Id
    String,     // "String"
    Integer,    // 1

    //Types
    Type,  // String | Int | Bool
    Array, // Array initialization word

    // Keywords.
    False,     // false
    For,       // for
    If,        // if
    Print,     // print
    Read,      // read
    True,      // true
    Var,       // var
    In,        // in
    Do,        // do
    End,       // end
    Begin,     // begin
    Assert,    // assert
    Program,   // program
    Function,  // function
    Procedure, // procedure
    Of,        // array [<int>] *of* int

    // End Of Files And Whitespaces
    Eof,     // End of File
    Whites,  // ' ' '\t' '\n' ..
    Comment, // {* ... *}
    Eol,     // End of line

    InitParser, // special token to initialize parser
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: String,
    pub position: Position,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::LeftParen => "(",
                Kind::RightParen => ")",
                Kind::LeftSquare => "[",
                Kind::RightSquare => "]",
                Kind::Minus => "-",
                Kind::Plus => "+",
                Kind::Semicolon => ";",
                Kind::Slash => "/",
                Kind::Star => "*",
                Kind::Bang => "!",
                Kind::And => "&",
                Kind::Equal => "=",
                Kind::Colon => ":",
                Kind::ColonEqual => ":=",
                Kind::Greater => "",
                Kind::GreaterEqual => "",
                Kind::Less => "<",
                Kind::LessEqual => "<=",
                Kind::Ddot => "..",
                Kind::Identifier => "Identifier",
                Kind::String => "\"String\"",
                Kind::Integer => "integer",
                Kind::Type => "int | string | bool",
                Kind::False => "false",
                Kind::For => "for",
                Kind::If => "if",
                Kind::Print => "print",
                Kind::Read => "read",
                Kind::True => "true",
                Kind::Var => "var",
                Kind::In => "in",
                Kind::Do => "do",
                Kind::Begin => "begin",
                Kind::End => "end",
                Kind::Assert => "assert",
                Kind::Eof => "End Of File",
                Kind::Whites => " ",
                Kind::Comment => "// or /* */",
                Kind::Eol => "End of Line",
                Kind::InitParser => "",
                Kind::Program => "Program",
                Kind::Function => "function",
                Kind::Procedure => "procedure",
                Kind::Array => "array",
                Kind::Of => "of",
            }
        )
    }
}
