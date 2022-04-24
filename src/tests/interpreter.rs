use crate::{
    core::{ast::ASTNode, variable::Type},
    interpreter::Interpreter,
    parser::Parser,
};

#[test]
fn correct_program_gets_interpreted_trough_run() {
    Interpreter::new(false)
        .run("var x : int; for x in 0..10 do print x; end for; assert(x = 11);".to_string());
}

#[test]
fn incorrect_program_prints_errors() {
    Interpreter::new(false)
        .run("var x : int; for x in 0..10 do; print x; end for; assert(x = 11);".to_string());
}

#[test]
fn correct_ast_gets_interpreted_correctly() {
    let mut parser =
        Parser::new("var x : int; for x in 0..10 do print x; end for; assert(x = 11);".to_string());
    let mut interpreter = Interpreter::new(false);
    interpreter.eval(parser.parse().unwrap());
}

#[test]
fn var_declaration_ok() {
    let mut parser = Parser::new("var x : string := \"variable\";".to_string());
    match parser.parse_statement() {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!(Type::String, node.var_type);
            assert_eq!(
                "variable",
                Interpreter::new(false)
                    .eval_expression_statement(node.value.unwrap())
                    .unwrap()
                    .string_value
            );
            assert_eq!("x", node.id.lexeme);
        }
        Ok(other) => panic!("Expected VarDeclNode, got {}", other),
        Err(e) => panic!("Expected VarDeclNode, got error {:?}", e),
    }
}

#[test]
fn var_declaration_already_declared() {
    let mut parser = Parser::new("var x : string := \"variable\"; var x : bool;".to_string());
    match parser.parse_statement() {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!(Type::String, node.var_type);
            assert_eq!(
                "variable",
                Interpreter::new(false)
                    .eval_expression_statement(node.value.unwrap())
                    .unwrap()
                    .string_value
            );
            assert_eq!("x", node.id.lexeme);
        }
        Ok(other) => panic!("Expected VarDeclNode, got {}", other),
        Err(e) => panic!("Expected VarDeclNode, got error {:?}", e),
    }
    if let Ok(node) = parser.parse_statement() {
        panic!("Expected error, got node {}", node);
    }
}

#[test]
fn var_declaration_no_value() {
    let mut parser = Parser::new("var x : string; var x : int;".to_string());
    match parser.parse_statement() {
        Ok(ASTNode::VariableDecl(node)) => {
            assert_eq!(Type::String, node.var_type);
            assert!(node.value.is_none());
            assert_eq!("x", node.id.lexeme);
        }
        Ok(other) => panic!("Expected VarDeclNode, got {}", other),
        Err(e) => panic!("Expected VarDeclNode, got error {:?}", e),
    }
    if let Ok(node) = parser.parse_statement() {
        panic!("Expected error, got node {}", node);
    }
}

#[test]
fn var_declaration_mismatch_types() {
    let mut parser = Parser::new("var x : string := 5;".to_string());
    let node = parser.parse_statement().unwrap();
    match node {
        ASTNode::VariableDecl(child) => {
            if let Ok(var) = Interpreter::new(false).eval_var_declaration(child) {
                panic!(
                    "Expected error, got correct execution with result {}",
                    var.string_value
                )
            }
        }
        other => panic!("Expected variable declaration node, got {}", other),
    }
}

#[test]
fn for_loop_stmt_tries_to_reassign_increment_error() {
    let mut parser = Parser::new("var x : int; for x in 1..10 do x := 3; end for;".to_string());
    let mut interpreter = Interpreter::new(false);

    let var_decl = parser.parse_statement().unwrap();
    let for_loop = parser.parse_statement().unwrap();

    match var_decl {
        ASTNode::VariableDecl(v_decl_node) => match for_loop {
            ASTNode::ForStmt(for_node) => match interpreter.eval_var_declaration(v_decl_node) {
                Ok(_) => {
                    if interpreter.eval_for(for_node).is_ok() {
                        panic!("Expected error, got evaluation success")
                    }
                }

                Err(e) => panic!(
                    "Expected var declaration to be successful, got errors {:?}",
                    e
                ),
            },
            other => panic!("Expected for node, found {}", other),
        },
        other => panic!("Expected var declaration node, found {}", other),
    }
}

#[test]
fn for_loop_stmt_tries_to_read_increment_error() {
    let mut parser = Parser::new("var x : int; for x in 1..10 do read x; end for;".to_string());
    let mut interpreter = Interpreter::new(false);

    let var_decl = parser.parse_statement().unwrap();
    let for_loop = parser.parse_statement().unwrap();

    match var_decl {
        ASTNode::VariableDecl(v_decl_node) => match for_loop {
            ASTNode::ForStmt(for_node) => match interpreter.eval_var_declaration(v_decl_node) {
                Ok(_) => {
                    if interpreter.eval_for(for_node).is_ok() {
                        panic!("Expected error, got evaluation success")
                    }
                }

                Err(e) => panic!(
                    "Expected var declaration to be successful, got errors {:?}",
                    e
                ),
            },
            other => panic!("Expected for node, found {}", other),
        },
        other => panic!("Expected var declaration node, found {}", other),
    }
}

#[test]
fn for_loop_stmt_nested_same_increment_error() {
    let mut parser = Parser::new(
        "var x : int; for x in 1..10 do for x in 0..10 do print x; end for; end for;".to_string(),
    );
    let mut interpreter = Interpreter::new(false);

    let var_decl = parser.parse_statement().unwrap();
    let for_loop = parser.parse_statement().unwrap();

    match var_decl {
        ASTNode::VariableDecl(v_decl_node) => match for_loop {
            ASTNode::ForStmt(for_node) => match interpreter.eval_var_declaration(v_decl_node) {
                Ok(_) => {
                    if interpreter.eval_for(for_node).is_ok() {
                        panic!("Expected error, got evaluation success")
                    }
                }

                Err(e) => panic!(
                    "Expected var declaration to be successful, got errors {:?}",
                    e
                ),
            },
            other => panic!("Expected for node, found {}", other),
        },
        other => panic!("Expected var declaration node, found {}", other),
    }
}

#[test]
fn for_loop_stmt_infalid_from_type_increment_error() {
    let mut parser =
        Parser::new("var x : int; for x in \"a\"..\"z\" do print x; end for;".to_string());
    let mut interpreter = Interpreter::new(false);

    let var_decl = parser.parse_statement().unwrap();
    let for_loop = parser.parse_statement().unwrap();

    match var_decl {
        ASTNode::VariableDecl(v_decl_node) => match for_loop {
            ASTNode::ForStmt(for_node) => match interpreter.eval_var_declaration(v_decl_node) {
                Ok(_) => {
                    if interpreter.eval_for(for_node).is_ok() {
                        panic!("Expected error, got evaluation success")
                    }
                }

                Err(e) => panic!(
                    "Expected var declaration to be successful, got errors {:?}",
                    e
                ),
            },
            other => panic!("Expected for node, found {}", other),
        },
        other => panic!("Expected var declaration node, found {}", other),
    }
}

#[test]
fn for_loop_stmt_invalid_increment_type_error() {
    let mut parser = Parser::new("var x : string; for x in 1..10 do print x; end for;".to_string());
    let mut interpreter = Interpreter::new(false);

    let var_decl = parser.parse_statement().unwrap();
    let for_loop = parser.parse_statement().unwrap();

    match var_decl {
        ASTNode::VariableDecl(v_decl_node) => match for_loop {
            ASTNode::ForStmt(for_node) => match interpreter.eval_var_declaration(v_decl_node) {
                Ok(_) => {
                    if interpreter.eval_for(for_node).is_ok() {
                        panic!("Expected error, got evaluation success")
                    }
                }

                Err(e) => panic!(
                    "Expected var declaration to be successful, got errors {:?}",
                    e
                ),
            },
            other => panic!("Expected for node, found {}", other),
        },
        other => panic!("Expected var declaration node, found {}", other),
    }
}

#[test]
fn var_reassignment_ok() {
    let mut parser = Parser::new("var x : int; x := 10;".to_string());
    let mut interpreter = Interpreter::new(false);

    match parser.parse_statement().unwrap() {
        ASTNode::VariableDecl(decl_node) => match parser.parse_statement().unwrap() {
            ASTNode::VarReassignment(reass_node) => {
                match interpreter.eval_var_declaration(decl_node) {
                    Ok(_) => {
                        if let Err(e) = interpreter.eval_var_reassignment(reass_node) {
                            panic!("Expected valid evaluation of variable reassignment, got errors {:?}",e)
                        }
                    }
                    Err(e) => panic!(
                        "Expected valid evaluation of variable declaration, got errors {:?}",
                        e
                    ),
                }
            }
            other => panic!("Expected variable reassignment node, got {}", other),
        },
        other => panic!("Expected var declaration node, got {}", other),
    }
}

#[test]
fn var_reassignment_wrong_type() {
    let mut parser = Parser::new("var x : int; x := true;".to_string());
    let mut interpreter = Interpreter::new(false);

    match parser.parse_statement().unwrap() {
        ASTNode::VariableDecl(decl_node) => match parser.parse_statement().unwrap() {
            ASTNode::VarReassignment(reass_node) => {
                match interpreter.eval_var_declaration(decl_node) {
                    Ok(_) => {
                        if interpreter.eval_var_reassignment(reass_node).is_ok() {
                            panic!("Expected invalid assign of variable, got correct evaluation")
                        }
                    }
                    Err(e) => panic!(
                        "Expected valid evaluation of variable declaration, got errors {:?}",
                        e
                    ),
                }
            }
            other => panic!("Expected variable reassignment node, got {}", other),
        },
        other => panic!("Expected var declaration node, got {}", other),
    }
}
