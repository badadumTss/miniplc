use crate::scanner::Scanner;

#[test]
fn scanner_error_if_eof_before_closing_comment() {
    let mut scanner = Scanner::new("/* This is an unclosed comment".to_string());
    if let Ok(token) = scanner.next_token() {
        panic!("Expected error, got token {}", token)
    }
}

#[test]
fn scanner_error_if_eof_before_closing_comment_branch2() {
    let mut scanner = Scanner::new("/* This is an unclosed comment*".to_string());
    if let Ok(token) = scanner.next_token() {
        panic!("Expected error, got token {}", token)
    }
}

#[test]
fn scanner_error_if_single_dot() {
    let mut scanner = Scanner::new(".;".to_string());
    if let Ok(token) = scanner.next_token() {
        panic!("Expected error, got token {}", token)
    }
}

#[test]
fn scanner_error_if_single_dot_branch2() {
    let mut scanner = Scanner::new("; .".to_string());
    let _unused = scanner.next_token();
    if let Ok(token) = scanner.next_token() {
        panic!("Expected error, got token {}", token)
    }
}
