use crate::{
    core::{ast::ASTNode, variable::Type},
    interpreter::Interpreter,
    parser::Parser,
};

#[test]
pub fn parse_emits_program() {
    let mut parser = Parser::new("var x : string := \"string value\";".to_string());
    match parser.parse() {
        Ok(ASTNode::Program(_)) => {}
        Ok(_) => panic!("Something other than a program node came out"),
        Err(_) => panic!("Errors emitted..."),
    }
}

#[test]
pub fn var_declaration_emits_var_node() {
    let mut parser = Parser::new("var x : string := \"string value\";".to_string());
    if let ASTNode::VariableDecl(node) = parser.parse_statement().unwrap() {
        assert_eq!("x", node.id.lexeme);
        assert_eq!(Type::String, node.var_type);
        assert_eq!(
            "string value",
            Interpreter::new(false)
                .eval_expression_statement(node.value.unwrap())
                .unwrap()
                .string_value
        )
    } else {
        panic!("parse_var_declaration doesn't emit a variableDecl node")
    }
}

#[test]
pub fn var_reassignment_error_if_not_init() {
    let mut parser = Parser::new("x := 3".to_string());
    if parser.parse().is_ok() {
        panic!("How does this work?!");
    }
}

#[test]
pub fn var_decl_of_already_declared_fails() {
    let mut parser = Parser::new("var x : int; var x : string".to_string());
    if parser.parse().is_ok() {
        panic!("How does this work?!");
    }
}

#[test]
pub fn valid_expression_gets_parsed_correctly() {
    let mut parser = Parser::new("3 + 1 * 8;".to_string());
    match parser.parse_expression() {
        Ok(ASTNode::ExpressionStmt(node)) => assert_eq!(
            11,
            Interpreter::new(false)
                .eval_expression_statement(node)
                .unwrap()
                .int_value
        ),
        _ => panic!("Expression fail"),
    };
}

#[test]
pub fn boolean_equality_gets_parsed_correctly() {
    let mut parser = Parser::new("1 = 1".to_string());
    match parser.parse_expression() {
        Ok(ASTNode::ExpressionStmt(node)) => assert!(
            Interpreter::new(false)
                .eval_expression_statement(node)
                .unwrap()
                .bool_value
        ),
        _ => panic!("Boolean expression not parsed correctly"),
    }
}

#[test]
pub fn boolean_expression_gets_parsed_correctly() {
    let mut parser = Parser::new("true & true".to_string());
    match parser.parse_expression() {
        Ok(ASTNode::ExpressionStmt(node)) => assert!(
            Interpreter::new(false)
                .eval_expression_statement(node)
                .unwrap()
                .bool_value
        ),
        Ok(_) => panic!("Boolean expression not parsed correctly, got something that is not an expression statement"),
	Err(e) => panic!("Boolean expression not parsed correctly, got a syntax error: {:?}", e),
    }
}

#[test]
pub fn assert_works_as_expected() {
    let mut parser = Parser::new("assert(false);".to_string());
    match parser.parse_statement() {
	Ok(ASTNode::AssertStmt(node)) =>  assert!(!Interpreter::new(false).eval_assert(node).unwrap().bool_value),
	Ok(_) => panic!("Boolean expression not parsed correctly, got something that is not an assert statement"),
	Err(e) => panic!("Boolean expression not parsed correctly, got a syntax error: {:?}", e),
    }
}

#[test]
pub fn line_comments_are_skipped() {
    let mut parser = Parser::new("//this is a comment\n".to_string());
    match parser.parse_statement() {
        Ok(ASTNode::EofStmt(_)) => {}
        Ok(node) => panic!("Should have gotten EOF, got node {}", node),
        Err(err) => panic!("Should have gotten EOF, got errors {:?}", err),
    }
}

#[test]
pub fn block_comments_are_skipped() {
    let mut parser = Parser::new("/*all of this is skipped*/".to_string());
    match parser.parse_statement() {
        Ok(ASTNode::EofStmt(_)) => {}
        Ok(node) => panic!("Should have gotten EOF, got node {}", node),
        Err(err) => panic!("Should have gotten EOF, got errors {:?}", err),
    }
}

#[test]
fn for_loop_correct() {
    let mut parser = Parser::new("var x : int; for x in 0..10 do print x; end for;".to_string());
    let var_node = parser.parse_statement();
    match var_node {
        Ok(ASTNode::VariableDecl(_)) => {}
        Ok(other) => panic!("Should have declared a variable, didnt, got: {}", other),
        Err(e) => panic!("Should have initialized variable, actually didnt: {:?}", e),
    }
    let for_node = parser.parse_statement();

    match for_node {
        Ok(ASTNode::ForStmt(node)) => {
            assert_eq!("x", node.increment.lexeme);
            let start = Interpreter::new(false)
                .eval_expression_statement(*(node.range_start))
                .unwrap();
            let end = Interpreter::new(false)
                .eval_expression_statement(*(node.range_end))
                .unwrap();

            assert_eq!(0, start.int_value);
            assert_eq!(10, end.int_value);
            match node.statements.iter().next() {
                Some(ASTNode::PrintStmt(_)) => {}
                Some(node) => panic!("Expected print stmt as child, got {}", node),
                None => panic!("Expected print stmt as child, got nothing"),
            }
        }
        Ok(node) => panic!("Expected for node, got {}", node),
        Err(e) => panic!("Expected for node, got error {:?}", e),
    }
}

#[test]
fn for_loop_missing_variable_error() {
    let mut parser = Parser::new("for x in 0..10 do print x; end for;".to_string());
    let for_node = parser.parse_statement();
    if let Ok(e) = for_node {
        panic!("Expected error, got node {}", e)
    }
}

#[test]
fn for_missing_semicolon_error() {
    let mut parser = Parser::new("var x : int; for x in 0..10 do print x; end for".to_string());
    let _var_decl = parser.parse_statement();
    let for_node = parser.parse_statement();
    if let Ok(e) = for_node {
        panic!("Expected error, got node {}", e)
    }
}

#[test]
fn for_internal_statements_error() {
    let mut parser = Parser::new("var x : int; for x in 0..10 do read y; end for;".to_string());
    let _var_decl = parser.parse_statement();
    let for_node = parser.parse_statement();
    if let Ok(e) = for_node {
        panic!("Expected error, got node {}", e)
    }
}

#[test]
fn for_start_expression_error() {
    let mut parser = Parser::new("var x : int; for x in var..10 do read y; end for;".to_string());
    let _var_decl = parser.parse_statement();
    let for_node = parser.parse_statement();
    if let Ok(e) = for_node {
        panic!("Expected error, got node {}", e)
    }
}

#[test]
fn for_end_expression_error() {
    let mut parser = Parser::new("var x : int; for x in 0..; do read y; end for;".to_string());
    let _var_decl = parser.parse_statement();
    let for_node = parser.parse_statement();
    if let Ok(e) = for_node {
        panic!("Expected error, got node {}", e)
    }
}

#[test]
fn read_returns_valid_ast() {
    let mut parser = Parser::new("var x : int; read x;".to_string());
    let _var_node = parser.parse_statement();
    let read_node = parser.parse_statement();
    match read_node {
        Ok(ASTNode::ReadStmt(node)) => {
            assert_eq!("x", node.variable_to_read_in.lexeme);
        }
        Ok(other) => panic!("Expected ASTNode::ReadStmt, got {}", other),
        Err(e) => panic!("Expected ASTNode::ReadStmt, got error {:?}", e),
    }
}

#[test]
fn read_returns_error_if_var_not_initialized() {
    let mut parser = Parser::new("read x;".to_string());
    let read_node = parser.parse_statement();
    if let Ok(other) = read_node {
        panic!("Expected ASTNode::ReadStmt, got {}", other)
    }
}

#[test]
fn var_reasssignment_valid_generates_valid_ast_node() {
    let mut parser = Parser::new("var x : int; x := 5;".to_string());
    let _init = parser.parse_statement();
    let assign = parser.parse_statement();
    match assign {
        Ok(ASTNode::VarReassignment(node)) => {
            assert_eq!("x", node.variable_to_reassign.lexeme);
            assert_eq!(
                5,
                Interpreter::new(false)
                    .eval_expression_statement(node.new_value)
                    .unwrap()
                    .int_value
            );
        }
        Ok(other) => panic!("Expected ASTNode::VarReassignment, got {}", other),
        Err(e) => panic!("Expected ASTNode::VarReassignment, got {:?}", e),
    }
}

#[test]
fn var_reasssignment_invalid_generates_error() {
    let mut parser = Parser::new("x := \"String\";".to_string());
    let assign = parser.parse_statement();
    if let Ok(what) = assign {
        panic!("Expected error, got {}", what)
    }
}

#[test]
fn invalid_statement_returns_error() {
    let mut parser = Parser::new(":= \"String\";".to_string());
    let err = parser.parse_statement();
    if let Ok(what) = err {
        panic!("Expected error, got {}", what)
    }
}

#[test]
fn print_that_does_not_make_sense_returns_error() {
    let mut parser = Parser::new("print var x;".to_string());
    let err = parser.parse_statement();
    if let Ok(what) = err {
        panic!("Expected error, got {}", what)
    }
}

#[test]
fn bool_bang_expression_ok() {
    let mut parser = Parser::new("var x : bool := !false;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_greater_expression_ok() {
    let mut parser = Parser::new("var x : bool := 1 > 0;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_greater_eq_graeater_expression_ok() {
    let mut parser = Parser::new("var x : bool := 1 >= 0;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_greater_eq_eq_expression_ok() {
    let mut parser = Parser::new("var x : bool := 1 >= 1;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_greater_eq_lower_expression_ok() {
    let mut parser = Parser::new("var x : bool := 0 >= 1;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(!value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_lower_eq_graeater_expression_ok() {
    let mut parser = Parser::new("var x : bool := 1 <= 0;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(!value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_lower_eq_eq_expression_ok() {
    let mut parser = Parser::new("var x : bool := 1 <= 1;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}

#[test]
fn bool_lower_eq_lower_expression_ok() {
    let mut parser = Parser::new("var x : bool := 0 <= 1;".to_string());
    let x = parser.parse_statement();
    match x {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!("x", node.id.lexeme);
            match Interpreter::new(false).eval_expression_statement(node.value.unwrap()) {
                Ok(value) => assert!(value.bool_value),
                Err(e) => panic!("Expected successful evaluation, got errors {:?}", e),
            }
        }
        Ok(other) => panic!("expected valid node, found {}", other),
        Err(vecc) => panic!("expected valid node, found {:?}", vecc),
    }
}
