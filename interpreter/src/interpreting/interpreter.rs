use crate::parsing::ASTNode;
use crate::parsing::parser::{ASTNode, ASTNodeType};

/// This struct provides an interpreter for Nack ASTs.
pub struct NackInterpreter {}

impl NackInterpreter {
    /// Creates a new interpreter.
    pub fn new() -> NackInterpreter {
        NackInterpreter {}
    }

    /// Interprets and executes the given program AST.
    pub fn interpret_ast(mut self, ast: &ASTNode) -> Result<(), InterpreterError> {
        for program_unit_node in &ast.children {
            self.evaluate_program_unit_node(program_unit_node)?;
        }

        Ok(())
    }

    fn evaluate_program_unit_node(&mut self, node: &ASTNode) -> Result<(), InterpreterError> {
        if let ASTNodeType::Grouping(label) = node.node_type {
            Ok(())
        } else {
            Err(InterpreterError::Internal(format!("{} is not a valid node type for program units", node.node_type))
        }
    }
}

pub enum InterpreterError {
    Internal(String),
    Runtime,
}
