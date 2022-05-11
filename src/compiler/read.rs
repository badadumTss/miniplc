use crate::core::{
    ast::{ASTNode, ReadStmtNode},
    types::{SimpleType, Type},
};

use super::Compiler;

impl Compiler {
    pub fn compile_read(&mut self, node: ReadStmtNode) {
        let where_to_read = match node.variable_to_read_in.as_ref().clone() {
            ASTNode::VarName(inode) => match inode.r_type {
                Type::Simple(s) => match s {
                    SimpleType::Int => format!("&{}", inode.id.lexeme),
                    SimpleType::String => inode.id.lexeme,
                    SimpleType::Bool => format!("&{}", inode.id.lexeme),
                    SimpleType::Void => format!("&{}", inode.id.lexeme),
                },
                Type::Array(_) => {
                    self.push_c_error(ASTNode::ReadStmt(node.clone()), "Unable to read into array");
                    "".to_string()
                }
            },
            ASTNode::ArrayRef(inode) => {
                self.compile_ast(inode.index.as_ref().clone());
                match inode.r_type {
                    Type::Simple(_) => format!("{}[last_int]", inode.array.lexeme),
                    Type::Array(_) => {
                        self.push_c_error(
                            ASTNode::ReadStmt(node.clone()),
                            "Unable to read into array",
                        );
                        "".to_string()
                    }
                }
            }
            _ => {
                self.push_c_error(
                    ASTNode::ReadStmt(node.clone()),
                    "Trying to read into an unknown variable",
                );
                "".to_string()
            }
        };
        let how_to_read = match node.variable_to_read_in.as_ref().clone().r_type() {
            Type::Simple(s) => match s {
                SimpleType::Int => "%d",
                SimpleType::String => "%s",
                SimpleType::Bool => "%d",
                SimpleType::Void => {
                    self.push_c_error(
                        ASTNode::ReadStmt(node.clone()),
                        "Trying to read into void expression",
                    );
                    "%d"
                }
            },
            Type::Array(_s) => {
                self.push_c_error(
                    ASTNode::ReadStmt(node.clone()),
                    "Trying to read into whole array",
                );
                "%d"
            }
        };
        self.push_instruction(format!(
            "last_{} = scanf(\"{}\", {})",
            node.variable_to_read_in.r_type().to_c_type(),
            how_to_read,
            where_to_read
        ));
    }
}
