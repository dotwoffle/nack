use crate::interpreting::values::{CoreNackValue, NackValue};
use crate::parsing::TRUE_KEYWORD;
use crate::parsing::ast::{
    ExpressionNode, ExpressionSubtreeRootNode, NackProgramAST, ProgramUnitNode,
};

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

    /// Evaluates a program unit subtree.
    fn evaluate_program_unit_node(
        &mut self,
        program_unit_node: &ProgramUnitNode,
    ) -> Result<(), InterpreterError> {
        match program_unit_node {
            ProgramUnitNode::Expression(expr_node) => {
                println!("{:?}", self.evaluate_expression_tree(expr_node)?.value);
                Ok(())
            }
        }
    }

    /// Evaluates an expression tree to produce a Nack value.
    fn evaluate_expression_tree(
        &mut self,
        expression_node: &ExpressionNode,
    ) -> Result<NackValue, InterpreterError> {
        match &expression_node.subtree_node {
            ExpressionSubtreeRootNode::IntLiteral(literal_node) => {
                let value = literal_node.token().value.parse().unwrap_or_else(|e| {
                    panic!(
                        "Failed to parse {} into int: {e}",
                        literal_node.token().value
                    )
                });
                Ok(NackValue {
                    value_type: String::from("Int"),
                    value: CoreNackValue::Int(value),
                })
            }
            ExpressionSubtreeRootNode::BoolLiteral(literal_node) => Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(literal_node.token().value == TRUE_KEYWORD),
            }),
            _ => todo!(),
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
    use crate::create_dummy_subtree_node;
    use crate::lexing::TokenKind;
    use crate::lexing::TokenKind::IntLiteral;
    use crate::parsing::ast::{BinaryOperatorNode, BoolLiteralNode, IntLiteralNode};
    use crate::test::{DUMMY_TOKEN_BOOL, DUMMY_TOKEN_INT, create_dummy_token};

    #[test]
    fn test_interpreting_expr_atoms_returns_correct_values() {
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&ExpressionNode {
                subtree_node: create_dummy_subtree_node!(
                    IntLiteral,
                    IntLiteralNode,
                    DUMMY_TOKEN_INT
                )
            }),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(123)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&ExpressionNode {
                subtree_node: create_dummy_subtree_node!(
                    BoolLiteral,
                    BoolLiteralNode,
                    DUMMY_TOKEN_BOOL
                )
            }),
            Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(true)
            })
        );
    }

    #[test]
    fn test_built_in_operators_return_correct_values() {
        fn build_binary_operator_tree(
            lhs: &str,
            rhs: &str,
            operator_type: TokenKind,
        ) -> ExpressionNode {
            ExpressionNode {
                subtree_node: ExpressionSubtreeRootNode::BinaryOperator(Box::new(
                    BinaryOperatorNode::try_new(
                        create_dummy_subtree_node!(
                            IntLiteral,
                            IntLiteralNode,
                            create_dummy_token(IntLiteral, lhs)
                        ),
                        create_dummy_subtree_node!(
                            IntLiteral,
                            IntLiteralNode,
                            create_dummy_token(IntLiteral, rhs)
                        ),
                        create_dummy_token(
                            operator_type,
                            match operator_type {
                                TokenKind::PlusSign => "+",
                                TokenKind::MinusSign => "-",
                                TokenKind::Asterisk => "*",
                                TokenKind::Slash => "/",
                                _ => panic!("Unknown built in operator type"),
                            },
                        ),
                    )
                    .unwrap_or_else(|e| panic!("{e}")),
                )),
            }
        }

        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree(
                "2",
                "2",
                TokenKind::PlusSign
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(4)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree(
                "2",
                "2",
                TokenKind::MinusSign
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(0)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree(
                "8",
                "5",
                TokenKind::Asterisk
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(40)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree(
                "6",
                "2",
                TokenKind::Slash
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(3)
            })
        );
    }
}
