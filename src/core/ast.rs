use std::fmt::Display;

use crate::core::token::*;
use crate::scanner::position::Position;

use super::objects::Object;
use super::symbol_table::SymbolTable;
use super::types::{SimpleType, Type};

#[derive(Clone, Debug)]
pub enum ASTNode {
    Program(ProgramNode),
    ProgramName(ProgramNameNode),
    FunctionDecl(FunctionDeclNode),
    ProcedureDecl(ProcedureDeclNode),
    Block(BlockNode),

    // Expressions
    BinaryExpression(BinaryExprNode),
    VarName(VariableNameExpressionNode),
    ArrayRef(ArrayRefExpr),
    Literal(LiteralExprNode),
    UnaryExpression(UnaryExprNode),
    VarReassignment(VarReassignmentExprNode),

    // Declarations
    VariableDecl(VariableDeclNode),

    // Statements
    WhileStmt(WhileStmtNode),
    IfStmt(IfStmtNode),
    ElseStmt(ElseStmtNode),
    PrintStmt(PrintStmtNode),
    ReadStmt(ReadStmtNode),
    AssertStmt(AssertStmtNode),
    FunctionCallStmt(FunctionCallNode),
    ProcedureCallStmt(ProcedureCallNode),
    ReturnStmt(ReturnStmtNode),

    // Void node for EOF
    EofStmt(EofNode),
}

impl ASTNode {
    pub fn r_type(&self) -> Type {
        match self {
            ASTNode::Program(_) => Type::Simple(SimpleType::Void),
            ASTNode::ProgramName(_) => Type::Simple(SimpleType::Void),
            ASTNode::FunctionDecl(_) => Type::Simple(SimpleType::Void),
            ASTNode::ProcedureDecl(_) => Type::Simple(SimpleType::Void),
            ASTNode::Block(_) => Type::Simple(SimpleType::Void),
            ASTNode::BinaryExpression(e) => e.r_type,
            ASTNode::VarName(i) => i.r_type,
            ASTNode::Literal(l) => l.r_type,
            ASTNode::UnaryExpression(u) => u.r_type,
            ASTNode::VarReassignment(_) => Type::Simple(SimpleType::Void),
            ASTNode::VariableDecl(_) => Type::Simple(SimpleType::Void),
            ASTNode::PrintStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::ReadStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::AssertStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::FunctionCallStmt(f) => f.r_type,
            ASTNode::ProcedureCallStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::ReturnStmt(r) => match r.clone().value {
                Some(v) => v.r_type(),
                None => Type::Simple(SimpleType::Void),
            },
            ASTNode::EofStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::WhileStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::ArrayRef(a) => a.r_type,
            ASTNode::IfStmt(_) => Type::Simple(SimpleType::Void),
            ASTNode::ElseStmt(_) => Type::Simple(SimpleType::Void),
        }
    }
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
    pub r_type: Type,
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
pub struct VariableNameExpressionNode {
    pub position: Position,
    pub id: Token,
    pub r_type: Type,
}

/// Node to rapresent a reference to an array
#[derive(Clone, Debug)]
pub struct ArrayRefExpr {
    pub position: Position,
    pub array: Token,
    pub index: Box<ASTNode>,
    pub r_type: Type,
}

/// Node to rapresent a literal
#[derive(Clone, Debug)]
pub struct LiteralExprNode {
    pub position: Position,
    pub value: Object,
    pub r_type: Type,
}

/// Node to rapresent a unary expression (Namely !<expression>)
#[derive(Clone, Debug)]
pub struct UnaryExprNode {
    pub position: Position,
    pub operand: Token,
    pub expression: Box<ASTNode>,
    pub r_type: Type,
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
    pub r_type: Type,
}

/// Node to rapresent a while statement
#[derive(Clone, Debug)]
pub struct WhileStmtNode {
    pub position: Position,
    pub guard: Box<ASTNode>,
    pub block: Box<ASTNode>,
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
    pub variable_to_reassign: Box<ASTNode>,
    pub new_value: Box<ASTNode>,
}

/// Node to rapresent a print statement
#[derive(Clone, Debug)]
pub struct PrintStmtNode {
    pub to_print: Box<ASTNode>,
    pub position: Position,
}

/// Node to rapresent a assert statement
#[derive(Clone, Debug)]
pub struct AssertStmtNode {
    pub position: Position,
    pub expr: Box<ASTNode>,
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
    pub args: Box<[ASTNode]>,
    pub target: String,
    pub r_type: Type,
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
    pub args: Box<[ASTNode]>,
    pub target: String,
}

#[derive(Clone, Debug)]
pub struct BlockNode {
    pub position: Position,
    pub context: SymbolTable,
    pub statements: Box<[ASTNode]>,
}

#[derive(Debug, Clone)]
pub struct IfStmtNode {
    pub position: Position,
    pub guard: Box<ASTNode>,
    pub then: Box<ASTNode>,
    pub else_stmt: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone)]
pub struct ElseStmtNode {
    pub position: Position,
    pub block: Box<ASTNode>,
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
            ASTNode::VarName(_) => write!(f, "identifier"),
            ASTNode::Literal(LiteralExprNode {
                position: _,
                value,
                r_type: actual_type,
            }) => write!(f, "literal: {} , {}", value, actual_type),
            ASTNode::UnaryExpression(_) => write!(f, "unary expression"),
            ASTNode::VarReassignment(_) => write!(f, "var reassignment"),
            ASTNode::VariableDecl(_) => write!(f, "variable declaraion"),
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
            ASTNode::WhileStmt(_) => write!(f, "while loop"),
            ASTNode::ArrayRef(_) => write!(f, "array reference"),
            ASTNode::IfStmt(_) => write!(f, "If statement"),
            ASTNode::ElseStmt(_) => write!(f, "else statement"),
        }
    }
}
