pub mod position;

use crate::core::errors::SyntaxError;
use crate::core::token::{Kind, Token};
use crate::scanner::position::Position;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line_num: usize,
    line_start: usize,
    init: bool,
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        let chars: Vec<char> = src.chars().collect();
        Scanner {
            source: chars,
            current: 0,
            line_num: 1,
            line_start: 0,
            init: true,
        }
    }

    /// Returns the current char the scanner is looking at in the
    /// source, None if the scanner is at the end
    pub fn get_current(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source[self.current])
    }

    /// Returns the next character with respect to the current one
    pub fn get_next(&self) -> Option<char> {
        if self.is_at_end() || self.source.len() <= (self.current + 1) {
            return None;
        }
        Some(self.source[self.current + 1])
    }

    /// True if the scanner is at the end, false otherwise
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advances to the next character
    pub fn advance(&mut self) -> Option<char> {
        if self.init {
            self.init = false;
            return self.get_current();
        }
        match self.get_current() {
            Some(_) => {
                self.current += 1;
                self.get_current()
            }
            None => None,
        }
    }

    /// Goes back by one character
    pub fn go_back(&mut self) {
        if !self.init {
            self.current -= 1;
        }
    }

    /// returns the ${num} line in the source code
    pub fn line(&self, num: usize) -> String {
        match self.source.iter().collect::<String>().lines().nth(num) {
            Some(stri) => stri.to_string(),
            None => String::new(),
        }
    }

    /// returns the line of the current character
    pub fn curr_line(&self) -> String {
        self.line(self.line_num - 1)
    }

    /// Generates a new token with the current position
    fn gen_token(&self, kind: Kind, lexeme: String) -> Token {
        Token {
            kind,
            lexeme,
            position: self.position(),
        }
    }

    /// Generates a Position structure based on internal
    /// rapresentation of position
    fn position(&self) -> Position {
        Position {
            char_number: self.current,
            line: self.line_num as i64,
            col: (self.current - self.line_start) as i64,
        }
    }

    /// Generates a token for a block comment in the source comment,
    /// recognises the regex /\*(?\*/)*\*/, with the unix regex
    /// specification this means any string that starts with /*,
    /// contains any character except for the group */ and finally the
    /// group */
    fn block_comment(&mut self) -> Result<Token, SyntaxError> {
        let mut comment = String::from("/*"); // the pattern that called this function
        loop {
            match self.advance() {
                Some(c) => match c {
                    '*' => match self.advance() {
                        Some(cn) => match cn {
                            '}' => {
                                break;
                            }
                            _ => {
                                comment.push(c);
                                comment.push(cn);
                            }
                        },
                        None => {
                            return Err(SyntaxError::new(
                                self.position(),
                                comment,
                                "Reached EOF before closing comment".to_string(),
                            ))
                        }
                    },
                    c => {
                        comment.push(c);
                    }
                },
                None => {
                    return Err(SyntaxError::new(
                        self.position(),
                        comment,
                        "Reached EOF before closing comment".to_string(),
                    ))
                }
            }
        }
        Ok(self.gen_token(Kind::Comment, comment.to_string()))
    }

    /// Recognises a string pattern with the regex "[^"]*" and escapes
    /// it
    fn string(&mut self) -> Result<Token, SyntaxError> {
        let mut string: String = String::new();
        loop {
            match self.advance() {
                Some(c) => {
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                None => {
                    return Err(SyntaxError::new(
                        self.position(),
                        string,
                        "Reached EOF while scanning string".to_string(),
                    ))
                }
            }
        }

        // Escape the string
        let bn = String::from("\\n");
        let bt = String::from("\\t");
        let br = String::from("\\r");
        let bs = String::from("\\");
        let qu = String::from("\\\"");
        let sq = String::from("\\\'");

        string = string.replace(bn.as_str(), "\n");
        string = string.replace(bt.as_str(), "\t");
        string = string.replace(br.as_str(), "\r");
        string = string.replace(qu.as_str(), "\"");
        string = string.replace(sq.as_str(), "\'");

        string = string.replace(bs.as_str(), "\\");

        Ok(self.gen_token(Kind::LitString, string))
    }

    /// Recognises any digit sequence with the pattern [0-9]*
    fn digits(&mut self) -> Result<Token, SyntaxError> {
        let mut digit: String = String::new();
        digit.push(self.get_current().unwrap()); //digit that triggered the function
        let mut is_real = false;
        let mut e_found = false;
        let mut sign_found = false;
        while let Some(c) = self.advance() {
            match c {
                c if c.is_digit(10) => digit.push(c),
                '.' => {
                    digit.push('.');
                    is_real = true;
                }
                'e' if is_real && !e_found => {
                    digit.push('e');
                    e_found = true;
                }
                '-' if is_real && !sign_found => {
                    digit.push('-');
                    sign_found = true;
                }
                _ => break,
            }
        }
        self.go_back();
        if is_real {
            Ok(self.gen_token(Kind::LitReal, digit))
        } else {
            Ok(self.gen_token(Kind::LitInt, digit))
        }
    }

    /// Recognises any one of the listed strings as tokens, if none is
    /// recognised but the function was called means the context
    /// required an identifier, therefore starts to recognise an
    /// identifier
    fn find_word(&mut self, word: String) -> Result<Token, SyntaxError> {
        match word.as_str() {
            "var" => Ok(self.gen_token(Kind::Var, word)),
            "in" => Ok(self.gen_token(Kind::In, word)),
            "do" => Ok(self.gen_token(Kind::Do, word)),
            "end" => Ok(self.gen_token(Kind::End, word)),
            "begin" => Ok(self.gen_token(Kind::Begin, word)),
            "read" => Ok(self.gen_token(Kind::Read, word)),
            "writeln" => Ok(self.gen_token(Kind::Print, word)),
            "assert" => Ok(self.gen_token(Kind::Assert, word)),
            "int" => Ok(self.gen_token(Kind::TInt, word)),
            "real" => Ok(self.gen_token(Kind::TReal, word)),
            "string" => Ok(self.gen_token(Kind::TString, word)),
            "bool" => Ok(self.gen_token(Kind::TBool, word)),
            "if" => Ok(self.gen_token(Kind::If, word)),
            "then" => Ok(self.gen_token(Kind::Then, word)),
            "else" => Ok(self.gen_token(Kind::Else, word)),
            "false" => Ok(self.gen_token(Kind::False, word)),
            "true" => Ok(self.gen_token(Kind::True, word)),
            "program" => Ok(self.gen_token(Kind::Program, word)),
            "function" => Ok(self.gen_token(Kind::Function, word)),
            "procedure" => Ok(self.gen_token(Kind::Procedure, word)),
            "array" => Ok(self.gen_token(Kind::TArray, word)),
            "of" => Ok(self.gen_token(Kind::Of, word)),
            "return" => Ok(self.gen_token(Kind::Return, word)),
            "while" => Ok(self.gen_token(Kind::While, word)),
            "or" => Ok(self.gen_token(Kind::Or, word)),
            "and" => Ok(self.gen_token(Kind::And, word)),
            _ => Ok(self.gen_token(Kind::Identifier, word)),
        }
    }

    /// Recognises any regex [a-zA-Z0-9_] as a word, letting find_word
    /// to understand what it means
    fn words(&mut self) -> Result<Token, SyntaxError> {
        let mut word: String = String::new();
        word.push(self.get_current().unwrap()); // alphanumeric that triggered words
        while let Some(c) = self.advance() {
            match c {
                c if c.is_alphanumeric() || c == '_' => word.push(c),
                _ => break,
            }
        }
        self.go_back();
        self.find_word(word)
    }

    /// Scans next token
    fn scan_token(&mut self) -> Result<Token, SyntaxError> {
        if self.is_at_end() {
            return Ok(Token {
                kind: Kind::Eof,
                lexeme: "".to_string(),
                position: self.position(),
            });
        }
        match self.advance() {
            Some(c) => match c {
                /* whitespaces */
                '\n' => Ok(self.gen_token(Kind::Eol, c.to_string())),
                c if c.is_whitespace() => Ok(self.gen_token(Kind::Whites, c.to_string())),

                /* pure one-char tokens */
                '(' => Ok(self.gen_token(Kind::LeftParen, c.to_string())),
                ')' => Ok(self.gen_token(Kind::RightParen, c.to_string())),
                '-' => Ok(self.gen_token(Kind::Minus, c.to_string())),
                '+' => Ok(self.gen_token(Kind::Plus, c.to_string())),
                ';' => Ok(self.gen_token(Kind::Semicolon, c.to_string())),
                '*' => Ok(self.gen_token(Kind::Star, c.to_string())),
                '=' => Ok(self.gen_token(Kind::Equal, c.to_string())),
                '!' => Ok(self.gen_token(Kind::Bang, c.to_string())),
                '[' => Ok(self.gen_token(Kind::LeftSquare, c.to_string())),
                ']' => Ok(self.gen_token(Kind::RightSquare, c.to_string())),
                ',' => Ok(self.gen_token(Kind::Comma, c.to_string())),
                '/' => Ok(self.gen_token(Kind::Slash, c.to_string())),
                '.' => Ok(self.gen_token(Kind::Dot, c.to_string())),

                /* 2/1 character tokens */
                ':' => {
                    if let Some(nc) = self.get_next() {
                        match nc {
                            '=' => {
                                self.advance();
                                Ok(self.gen_token(Kind::ColonEqual, ":=".to_string()))
                            }
                            _ => Ok(self.gen_token(Kind::Colon, c.to_string())),
                        }
                    } else {
                        Ok(self.gen_token(Kind::Colon, c.to_string()))
                    }
                }

                '>' => {
                    if let Some(nc) = self.get_next() {
                        match nc {
                            '=' => {
                                self.advance();
                                Ok(self.gen_token(Kind::GreaterEqual, ">=".to_string()))
                            }
                            _ => Ok(self.gen_token(Kind::Greater, c.to_string())),
                        }
                    } else {
                        Ok(self.gen_token(Kind::Greater, c.to_string()))
                    }
                }

                '<' => {
                    if let Some(nc) = self.get_next() {
                        match nc {
                            '=' => {
                                self.advance();
                                Ok(self.gen_token(Kind::LessEqual, "<=".to_string()))
                            }
                            _ => Ok(self.gen_token(Kind::Less, c.to_string())),
                        }
                    } else {
                        Ok(self.gen_token(Kind::Less, c.to_string()))
                    }
                }

                /* comments */
                '{' => match self.get_next() {
                    Some(nc) => match nc {
                        '*' => self.block_comment(),
                        _ => Err(SyntaxError::new(
                            self.position(),
                            self.curr_line(),
                            format!("Unknown token: {}", nc),
                        )),
                    },
                    None => Err(SyntaxError::new(
                        self.position(),
                        self.curr_line(),
                        "Unexpected End Of File wile reading the source".to_string(),
                    )),
                },

                '"' => self.string(),

                c if { c.is_digit(10) } => self.digits(),

                c if { c.is_alphanumeric() } => self.words(),

                /* defaults to syntax error */
                _ => {
                    let mut erroneous = String::new();
                    erroneous.push(self.get_current().unwrap()); //exists since it triggered the branch
                    while let Some(nc) = self.advance() {
                        match nc {
                            nc if nc.is_whitespace() => break,
                            _ => erroneous.push(nc),
                        }
                    }
                    Err(SyntaxError::new(
                        self.position(),
                        self.curr_line(),
                        format! {"Unknown token: {}", erroneous},
                    ))
                }
            },
            None => Ok(Token {
                kind: Kind::Eof,
                lexeme: "".to_string(),
                position: self.position(),
            }),
        }
    }

    /// Returns next token based on the scan, wich could recognise
    /// also whitespace tokens
    pub fn next_token(&mut self) -> Result<Token, SyntaxError> {
        loop {
            /* invariant: at some point scan_token returns a token or
             * a syntax error, either a real token or a Eof since it
             * is at the end of file */
            match self.scan_token() {
                Ok(token) => match token.kind {
                    Kind::Eol => {
                        self.line_num += 1;
                        self.line_start = self.current + 1;
                    }
                    Kind::Whites | Kind::Comment => {}
                    _ => return Ok(token),
                },
                Err(syn_error) => return Err(syn_error),
            }
        }
    }
}
