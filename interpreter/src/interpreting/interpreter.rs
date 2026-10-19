use crate::interpreting::values::{CoreNackValue, NackValue};
use crate::parsing::TRUE_KEYWORD;
use crate::parsing::ast::{
    BinaryOperator, ExpressionNode, ExpressionSubtreeRootNode, NackProgramAST, ProgramUnitNode,
    UnaryOperator,
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
        self.evaluate_expression_subtree(&expression_node.subtree_node)
    }

    /// Evaluates an expression subtree to produce a Nack value.
    fn evaluate_expression_subtree(
        &mut self,
        subtree_root_node: &ExpressionSubtreeRootNode,
    ) -> Result<NackValue, InterpreterError> {
        //TODO turn operators into transforms
        macro_rules! extract_value_from_nack_value {
            ($pattern:path, $value:expr) => {
                match $value.value {
                    $pattern(value) => value,
                    _ => panic!("Nack value with unexpected type {:?}", $value.value),
                }
            };
        }

        match subtree_root_node {
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
            ExpressionSubtreeRootNode::BinaryOperator(operator_node) => {
                let lhs_value = self.evaluate_expression_subtree(&operator_node.lhs)?;
                let rhs_value = self.evaluate_expression_subtree(&operator_node.rhs)?;

                match operator_node.operator {
                    BinaryOperator::Addition => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Int"),
                            value: CoreNackValue::Int(lhs_value + rhs_value),
                        })
                    }
                    BinaryOperator::Subtraction => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Int"),
                            value: CoreNackValue::Int(lhs_value - rhs_value),
                        })
                    }
                    BinaryOperator::Multiplication => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Int"),
                            value: CoreNackValue::Int(lhs_value * rhs_value),
                        })
                    }
                    BinaryOperator::Division => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Int, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Int"),
                            value: CoreNackValue::Int(lhs_value / rhs_value),
                        })
                    }
                    BinaryOperator::LogicalAnd => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Bool, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Bool, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Bool"),
                            value: CoreNackValue::Bool(lhs_value && rhs_value),
                        })
                    }
                    BinaryOperator::LogicalOr => {
                        let lhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Bool, lhs_value);
                        let rhs_value =
                            extract_value_from_nack_value!(CoreNackValue::Bool, rhs_value);

                        Ok(NackValue {
                            value_type: String::from("Bool"),
                            value: CoreNackValue::Bool(lhs_value || rhs_value),
                        })
                    }
                }
            }
            ExpressionSubtreeRootNode::UnaryOperator(operator_node) => {
                let operand_value = self.evaluate_expression_subtree(&operator_node.expression)?;

                match operator_node.operator {
                    UnaryOperator::LogicalNot => {
                        let operand_value =
                            extract_value_from_nack_value!(CoreNackValue::Bool, operand_value);

                        Ok(NackValue {
                            value_type: String::from("Bool"),
                            value: CoreNackValue::Bool(!operand_value),
                        })
                    }
                }
            }
            ExpressionSubtreeRootNode::Identifier(_) => todo!(),
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
    use crate::lexing::TokenKind::{Identifier, IntLiteral};
    use crate::parsing::ast::{
        BinaryOperator, BinaryOperatorNode, BoolLiteralNode, IntLiteralNode, UnaryOperator,
        UnaryOperatorNode,
    };
    use crate::parsing::{AND_KEYWORD, FALSE_KEYWORD, NOT_KEYWORD, OR_KEYWORD};
    use crate::test::{
        DUMMY_TOKEN_BOOL, DUMMY_TOKEN_DIVIDE_SIGN, DUMMY_TOKEN_INT, DUMMY_TOKEN_MINUS_SIGN,
        DUMMY_TOKEN_MULT_SIGN, DUMMY_TOKEN_PLUS_SIGN, create_dummy_token,
    };

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
    fn test_built_in_binary_operators_return_correct_values() {
        macro_rules! build_binary_operator_tree {
            ($lhs:expr, $rhs:expr, $operator_token:expr, $operand_subtree_variant:ident, $operand_subtree_node_type:ident, $operand_token_type:ident) => {
                ExpressionNode {
                    subtree_node: ExpressionSubtreeRootNode::BinaryOperator(Box::new(
                        BinaryOperatorNode {
                            lhs: create_dummy_subtree_node!(
                                $operand_subtree_variant,
                                $operand_subtree_node_type,
                                create_dummy_token($operand_token_type, $lhs)
                            ),
                            rhs: create_dummy_subtree_node!(
                                $operand_subtree_variant,
                                $operand_subtree_node_type,
                                create_dummy_token($operand_token_type, $rhs)
                            ),
                            operator: BinaryOperator::from(&$operator_token),
                        },
                    )),
                }
            };
        }

        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                "2",
                "2",
                *DUMMY_TOKEN_PLUS_SIGN,
                IntLiteral,
                IntLiteralNode,
                IntLiteral
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(4)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                "2",
                "2",
                *DUMMY_TOKEN_MINUS_SIGN,
                IntLiteral,
                IntLiteralNode,
                IntLiteral
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(0)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                "8",
                "5",
                *DUMMY_TOKEN_MULT_SIGN,
                IntLiteral,
                IntLiteralNode,
                IntLiteral
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(40)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                "6",
                "2",
                *DUMMY_TOKEN_DIVIDE_SIGN,
                IntLiteral,
                IntLiteralNode,
                IntLiteral
            )),
            Ok(NackValue {
                value_type: String::from("Int"),
                value: CoreNackValue::Int(3)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                TRUE_KEYWORD,
                FALSE_KEYWORD,
                create_dummy_token(Identifier, AND_KEYWORD),
                BoolLiteral,
                BoolLiteralNode,
                Identifier
            )),
            Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(false)
            })
        );
        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_binary_operator_tree!(
                TRUE_KEYWORD,
                FALSE_KEYWORD,
                create_dummy_token(Identifier, OR_KEYWORD),
                BoolLiteral,
                BoolLiteralNode,
                Identifier
            )),
            Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(true)
            })
        );
    }

    #[test]
    fn test_built_in_unary_operators_return_correct_values() {
        macro_rules! build_unary_operator_tree {
            ($operand:expr, $operator_token:expr, $operand_subtree_variant:ident, $operand_subtree_node_type:ident, $operand_token_type:ident) => {
                ExpressionNode {
                    subtree_node: ExpressionSubtreeRootNode::UnaryOperator(Box::new(
                        UnaryOperatorNode {
                            expression: create_dummy_subtree_node!(
                                $operand_subtree_variant,
                                $operand_subtree_node_type,
                                create_dummy_token($operand_token_type, $operand)
                            ),
                            operator: UnaryOperator::from(&$operator_token),
                        },
                    )),
                }
            };
        }

        assert_eq!(
            NackInterpreter::new().evaluate_expression_tree(&build_unary_operator_tree!(
                TRUE_KEYWORD,
                create_dummy_token(Identifier, NOT_KEYWORD),
                BoolLiteral,
                BoolLiteralNode,
                Identifier
            )),
            Ok(NackValue {
                value_type: String::from("Bool"),
                value: CoreNackValue::Bool(false)
            })
        );
    }
}
