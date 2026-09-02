use crate::interpreting::values::{CoreNackValue, NackValue};
use crate::lexing::TokenKind;
use crate::parsing::parser::{ASTNode, ASTNodeType};
use crate::parsing::{EXPRESSION_LABEL, FALSE_KEYWORD, TRUE_KEYWORD};

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

    /// Evaluates a single program unit node.
    fn evaluate_program_unit_node(
        &mut self,
        program_unit_node: &ASTNode,
    ) -> Result<(), InterpreterError> {
        if let ASTNodeType::Grouping(label) = program_unit_node.node_type {
            match label {
                EXPRESSION_LABEL => {
                    println!("{:?}", self.evaluate_expression(program_unit_node)?.value);
                    Ok(())
                }
                &_ => todo!(),
            }
        } else {
            panic!(
                "{:?} is not a valid node type for program units",
                program_unit_node.node_type
            )
        }
    }

    fn evaluate_expression(
        &mut self,
        expression_node: &ASTNode,
    ) -> Result<NackValue, InterpreterError> {
        let actual_root_node = match &expression_node.node_type {
            ASTNodeType::Grouping(label) if *label == EXPRESSION_LABEL => {
                &expression_node.children[0]
            }
            _ => expression_node,
        };

        match &actual_root_node.node_type {
            ASTNodeType::Grouping(label) => todo!(),
            ASTNodeType::Token(token) => match token.kind {
                TokenKind::IntLiteral => Ok(NackValue {
                    value_type: String::from("Int"),
                    value: CoreNackValue::Int(token.value.parse().unwrap_or_else(|e| {
                        panic!("Failed to parse {} into int: {e}", token.value)
                    })),
                }),
                TokenKind::Identifier => {
                    if token.value == TRUE_KEYWORD || token.value == FALSE_KEYWORD {
                        Ok(NackValue {
                            value_type: String::from("Bool"),
                            value: CoreNackValue::Bool(token.value == TRUE_KEYWORD),
                        })
                    } else {
                        todo!()
                    }
                }
                _ => panic!("Invalid expression node: {}", token.value),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Runtime,
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use crate::lexing::{SourcePosition, Token, TokenKind};
    use crate::parsing::parser::{ASTNode, ASTNodeType};

    #[test]
    fn test_interpreting_expr_atoms_returns_correct_values() {
        assert_eq!(
            NackInterpreter::new().evaluate_expression(&ASTNode {
                node_type: ASTNodeType::Token(Token {
                    position: SourcePosition { line: 0, column: 0 },
                    value: String::from("123"),
                    kind: TokenKind::IntLiteral,
                }),
                children: vec![],
            }),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(123)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression(&ASTNode {
                node_type: ASTNodeType::Token(Token {
                    position: SourcePosition { line: 0, column: 0 },
                    value: String::from("true"),
                    kind: TokenKind::Identifier,
                }),
                children: vec![],
            }),
            Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(true)
            })
        );
    }
}
