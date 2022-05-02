use std::fmt::Display;

use crate::core::token::*;
use crate::scanner::position::Position;

use super::objects::Object;
use super::symbol_table::SymbolTable;
use super::types::Type;

#[derive(Clone, Debug)]
pub enum ASTNode {
    Program(ProgramNode),
    ProgramName(ProgramNameNode),
    FunctionDecl(FunctionDeclNode),
    ProcedureDecl(ProcedureDeclNode),
    Block(BlockNode),

    // Expressions
    BinaryExpression(BinaryExprNode),
    Identifier(IdentifierExprNode),
    Literal(LiteralExprNode),
    UnaryExpression(UnaryExprNode),
    VarReassignment(VarReassignmentExprNode),

    // Declarations
    VariableDecl(VariableDeclNode),

    // Statements
    ExpressionStmt(ExpressionStmtNode),
    ForStmt(ForStmtNode),
    PrintStmt(PrintStmtNode),
    ReadStmt(ReadStmtNode),
    AssertStmt(AssertStmtNode),
    FunctionCallStmt(FunctionCallNode),
    ProcedureCallStmt(ProcedureCallNode),
    ReturnStmt(ReturnStmtNode),

    // Void node for EOF
    EofStmt(EofNode),
}

/// Node that rapresent a whole program, each statement is an ASTNode
/// in a Boxed buffer
#[derive(Clone, Debug)]
pub struct ProgramNode {
    pub program_name: ProgramNameNode,
    pub functions: Box<[FunctionDeclNode]>,
    pub procedures: Box<[ProcedureDeclNode]>,
    pub main_block: BlockNode,
}

/// Node that rapresents a binary expression (both binary and integer)
#[derive(Clone, Debug)]
pub struct BinaryExprNode {
    pub position: Position,
    pub left: Box<ASTNode>,
    pub op: Token,
    pub op_type: BinaryExprType,
    pub right: Box<ASTNode>,
}

/// Binary expression type, to know what to do with a binary
/// expression node
#[derive(Clone, Debug)]
pub enum BinaryExprType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LogicAND,
    LogicEQ,
    LogicGreaterThan,
    LogicGreaterThanEQ,
    LogicLessThan,
    LogicLessThanEQ,
}

impl Display for BinaryExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Node to rapresent an Identifier
#[derive(Clone, Debug)]
pub struct IdentifierExprNode {
    pub position: Position,
    pub id: Token,
}

/// Node to rapresent a literal
#[derive(Clone, Debug)]
pub struct LiteralExprNode {
    pub position: Position,
    pub value: Object,
    pub actual_type: Type,
}

/// Node to rapresent a unary expression (Namely !<expression>)
#[derive(Clone, Debug)]
pub struct UnaryExprNode {
    pub position: Position,
    pub operand: Token,
    pub expression: Box<ASTNode>,
}

/// Node that rapresent a statement to declare a variable
#[derive(Clone, Debug)]
pub struct VariableDeclNode {
    pub position: Position,
    pub id: Token,
    pub var_type: Type,
    pub value: Option<ExpressionStmtNode>,
}

/// Node to rapresent an expression, both unary and binary, when the
/// necessary elements to understand what to do are given by the
/// context
#[derive(Clone, Debug)]
pub struct ExpressionStmtNode {
    pub position: Position,
    pub child: Box<ASTNode>,
}

/// Node to rapresent a for statement
#[derive(Clone, Debug)]
pub struct ForStmtNode {
    pub position: Position,
    pub increment: Token,
    pub range_start: Box<ExpressionStmtNode>,
    pub range_end: Box<ExpressionStmtNode>,
    pub statements: Box<[ASTNode]>,
}

/// Node to rapresent a read statement
#[derive(Clone, Debug)]
pub struct ReadStmtNode {
    pub position: Position,
    pub variable_to_read_in: Token,
}

/// Node that rapresent a statement to assign a value to a variable
#[derive(Clone, Debug)]
pub struct VarReassignmentExprNode {
    pub position: Position,
    pub variable_to_reassign: Token,
    pub new_value: ExpressionStmtNode,
}

/// Node to rapresent a print statement
#[derive(Clone, Debug)]
pub struct PrintStmtNode {
    pub to_print: ExpressionStmtNode,
    pub position: Position,
}

/// Node to rapresent a assert statement
#[derive(Clone, Debug)]
pub struct AssertStmtNode {
    pub position: Position,
    pub expr: ExpressionStmtNode,
}

#[derive(Clone, Debug)]
pub struct FunctionDeclNode {
    pub name: String,
    pub position: Position,
    pub args: SymbolTable,
    pub block: Box<ASTNode>,
    pub r_type: Type,
}

#[derive(Clone, Debug)]
pub struct FunctionCallNode {
    pub position: Position,
    pub args: SymbolTable,
    pub target: String,
}

#[derive(Clone, Debug)]
pub struct ProcedureDeclNode {
    pub name: String,
    pub position: Position,
    pub args: SymbolTable,
    pub block: Box<ASTNode>,
}

#[derive(Clone, Debug)]
pub struct ProcedureCallNode {
    pub position: Position,
    pub args: SymbolTable,
    pub target: String,
}

#[derive(Clone, Debug)]
pub struct BlockNode {
    pub position: Position,
    pub context: SymbolTable,
    pub statements: Box<[ASTNode]>,
}

#[derive(Clone, Debug)]
pub struct ProgramNameNode {
    pub name: Token,
}

#[derive(Clone, Debug)]
pub struct ReturnStmtNode {
    pub token: Token,
    pub value: Option<Box<ASTNode>>,
}

#[derive(Clone, Debug)]
pub struct EofNode {
    pub eof: Token,
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Program(node) => write!(
                f,
                "program: {:#?},\nfunctions: {:#?},\nprocedures: {:#?},\nmain block: {:#?}",
                node.program_name, node.functions, node.procedures, node.main_block
            ),
            ASTNode::BinaryExpression(_) => write!(f, "binary expression"),
            ASTNode::Identifier(_) => write!(f, "identifier"),
            ASTNode::Literal(LiteralExprNode {
                position: _,
                value,
                actual_type,
            }) => write!(f, "literal: {} , {}", value, actual_type),
            ASTNode::UnaryExpression(_) => write!(f, "unary expression"),
            ASTNode::VarReassignment(_) => write!(f, "var reassignment"),
            ASTNode::VariableDecl(_) => write!(f, "variable declaraion"),
            ASTNode::ExpressionStmt(_) => write!(f, "expression"),
            ASTNode::ForStmt(_) => write!(f, "for loop"),
            ASTNode::PrintStmt(_) => write!(f, "print"),
            ASTNode::ReadStmt(_) => write!(f, "read"),
            ASTNode::AssertStmt(_) => write!(f, "assert"),
            ASTNode::EofStmt(_) => write!(f, "end of file"),
            ASTNode::FunctionDecl(_) => write!(f, "function"),
            ASTNode::ProcedureDecl(_) => write!(f, "procedure"),
            ASTNode::Block(_) => write!(f, "block"),
            ASTNode::ProgramName(_) => write!(f, "program name"),
            ASTNode::FunctionCallStmt(_) => write!(f, "function call"),
            ASTNode::ProcedureCallStmt(_) => write!(f, "procedure call"),
            ASTNode::ReturnStmt(_) => write!(f, "return statement"),
        }
    }
}
