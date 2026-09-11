use crate::interpreting::values::{CoreNackValue, NackValue};
use crate::parsing::ast::{
    ExpressionNode, ExpressionSubtreeRootNode, NackProgramAST, ProgramUnitNode,
};
use crate::parsing::{FALSE_KEYWORD, TRUE_KEYWORD};

/// This struct provides an interpreter for Nack ASTs.
pub struct NackInterpreter {}

impl NackInterpreter {
    /// Creates a new interpreter.
    pub fn new() -> NackInterpreter {
        NackInterpreter {}
    }

    /// Interprets and executes the given program AST.
    pub fn interpret_ast(mut self, ast: &NackProgramAST) -> Result<(), InterpreterError> {
        for program_unit_node in &ast.program_units {
            self.evaluate_program_unit_node(program_unit_node)?;
        }

        Ok(())
    }

    /// Evaluates a subtree with a `PROGRAM_UNIT` node at the root. If the root node is not a
    /// `PROGRAM_UNIT` node, or an error occurs during interpreting, an error is returned.
    fn evaluate_program_unit_node(
        &mut self,
        program_unit_node: &ProgramUnitNode,
    ) -> Result<(), InterpreterError> {
        match program_unit_node {
            ProgramUnitNode::Expression(expr_node) => Ok(println!(
                "{:?}",
                self.evaluate_expression_tree(expr_node)?.value
            )),
        }
    }

    /// Evaluates an expression subtree to produce a Nack value, including trees with an
    /// `EXPRESSION` root node. An error is returned if the interpreter encounters an error during
    /// evaluation.
    fn evaluate_expression_tree(
        &mut self,
        expression_node: &ExpressionNode,
    ) -> Result<NackValue, InterpreterError> {
        match &expression_node.subtree_node {
            ExpressionSubtreeRootNode::IntLiteral(literal_node) => {
                let value = literal_node.token.value.parse().unwrap_or_else(|e| {
                    panic!("Failed to parse {} into int: {e}", literal_node.token.value)
                });
                Ok(NackValue {
                    value_type: String::from("Int"),
                    value: CoreNackValue::Int(value),
                })
            }
            ExpressionSubtreeRootNode::BoolLiteral(literal_node) => {
                if literal_node.token.value == TRUE_KEYWORD
                    || literal_node.token.value == FALSE_KEYWORD
                {
                    Ok(NackValue {
                        value_type: String::from("Bool"),
                        value: CoreNackValue::Bool(literal_node.token.value == TRUE_KEYWORD),
                    })
                } else {
                    todo!()
                }
            }
        }
    }
}

/// This enum represents an error encountered during interpreting.
#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    /// An error encountered during runtime.
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
            NackInterpreter::new().evaluate_expression_tree(&ASTNode {
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
            NackInterpreter::new().evaluate_expression_tree(&ASTNode {
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
