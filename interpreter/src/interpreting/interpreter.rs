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
    use crate::parsing::ast::{BinaryOperatorNode, BoolLiteralNode, IntLiteralNode};
    use crate::test::{
        DUMMY_TOKEN_BOOL, DUMMY_TOKEN_INT, DUMMY_TOKEN_MULT_SIGN, DUMMY_TOKEN_PLUS_SIGN,
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
}
