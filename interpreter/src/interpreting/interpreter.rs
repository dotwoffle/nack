use crate::lexing::TokenKind;
use crate::parsing::EXPRESSION_LABEL;
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

    fn evaluate_program_unit_node(
        &mut self,
        program_unit_node: &ASTNode,
    ) -> Result<(), InterpreterError> {
        if let ASTNodeType::Grouping(label) = program_unit_node.node_type {
            Ok(())
        } else {
            Err(InterpreterError::Internal(format!(
                "{:?} is not a valid node type for program units",
                program_unit_node.node_type
            )))
        }
    }

    fn evaluate_expression(&mut self, expression_node: &ASTNode) -> Result<(), InterpreterError> {
        let actual_root_node = match &expression_node.node_type {
            ASTNodeType::Grouping(label) if *label == EXPRESSION_LABEL => {
                &expression_node.children[0]
            }
            _ => expression_node,
        };

        match &actual_root_node.node_type {
            ASTNodeType::Grouping(label) => todo!(),
            ASTNodeType::Token(token) => match token.kind {
                TokenKind::IntLiteral => todo!(),
                TokenKind::Identifier => todo!(),
                _ => Err(InterpreterError::Internal(format!(
                    "Invalid expression node: {}",
                    token.value
                ))),
            },
        }
    }
}

pub enum InterpreterError {
    Internal(String),
    Runtime,
}
