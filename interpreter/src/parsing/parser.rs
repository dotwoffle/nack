use crate::lexing::{SyntaxError, Token, TokenKind, TokenStream};
use crate::parsing::ast::{
    BinaryOperatorNode, BoolLiteralNode, ExpressionNode, ExpressionSubtreeRootNode, IdentifierNode,
    IntLiteralNode, NackProgramAST, ProgramUnitNode,
};
use crate::parsing::{FALSE_KEYWORD, TRUE_KEYWORD};

/// This struct provides a parser used to turn a series of Nack language tokens into an AST.
///
/// Example
/// ```rust
/// let tokens = vec![];
/// let parser = NackParser::new(tokens);
/// let ast_root = parser.parse();
/// ```
pub struct NackParser {
    /// The token stream being parsed.
    tokens: TokenStream,
}

impl NackParser {
    /// Creates a new parser prepared to parse the given list of tokens. The list of tokens cannot
    /// be empty.
    pub fn new(tokens: Vec<Token>) -> NackParser {
        assert!(!tokens.is_empty(), "Token stream cannot be empty");
        NackParser {
            tokens: TokenStream::new(tokens),
        }
    }

    /// Parses the stored token stream and produces an AST.
    pub fn parse(mut self) -> Result<NackProgramAST, SyntaxError> {
        self.handle_program_rule()
    }

    /// Parses the PROGRAM language rule and returns the root of the produced subtree.
    fn handle_program_rule(&mut self) -> Result<NackProgramAST, SyntaxError> {
        let mut program_units = vec![];

        while !self.tokens.next_token_has_types(&[TokenKind::Eof]) {
            program_units.push(self.handle_program_unit_rule()?);
        }

        Ok(NackProgramAST { program_units })
    }

    /// Parses the PROGRAM_UNIT language rule and returns the root of the produced subtree.
    fn handle_program_unit_rule(&mut self) -> Result<ProgramUnitNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier | TokenKind::IntLiteral => self
                .handle_expression_rule()
                .map(ProgramUnitNode::Expression),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }

    /// Parses the EXPRESSION language rule and returns the root of the produced subtree.
    fn handle_expression_rule(&mut self) -> Result<ExpressionNode, SyntaxError> {
        Ok(ExpressionNode {
            subtree_node: self.handle_binary_op_expression_rule(
                &[TokenKind::PlusSign, TokenKind::MinusSign],
                Self::handle_mult_expr_rule,
            )?,
        })
    }

    /// Parses the MULT_EXPR language rule and returns the root of the produced subtree.
    fn handle_mult_expr_rule(&mut self) -> Result<ExpressionSubtreeRootNode, SyntaxError> {
        self.handle_binary_op_expression_rule(
            &[TokenKind::Asterisk, TokenKind::Slash],
            Self::handle_expr_atom_rule,
        )
    }

    /// Parses the EXPR_ATOM language rule and returns the root of the produced subtree.
    fn handle_expr_atom_rule(&mut self) -> Result<ExpressionSubtreeRootNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier => {
                let identifier = &self.tokens.peek(0).value;

                if identifier == TRUE_KEYWORD || identifier == FALSE_KEYWORD {
                    Ok(ExpressionSubtreeRootNode::BoolLiteral(
                        BoolLiteralNode::try_from(self.tokens.pop())
                            .unwrap_or_else(|e| panic!("{e}")),
                    ))
                } else {
                    Ok(ExpressionSubtreeRootNode::Identifier(
                        IdentifierNode::try_from(self.tokens.pop())
                            .unwrap_or_else(|e| panic!("{e}")),
                    ))
                }
            }
            TokenKind::IntLiteral => Ok(ExpressionSubtreeRootNode::IntLiteral(
                IntLiteralNode::try_from(self.tokens.pop()).unwrap_or_else(|e| panic!("{e}")),
            )),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }

    fn handle_binary_op_expression_rule<F>(
        &mut self,
        operator_token_types: &[TokenKind],
        sub_grammar_rule: F,
    ) -> Result<ExpressionSubtreeRootNode, SyntaxError>
    where
        F: Fn(&mut NackParser) -> Result<ExpressionSubtreeRootNode, SyntaxError>,
    {
        let mut expr_root_node = sub_grammar_rule(self)?;

        while self.tokens.next_token_has_types(operator_token_types) {
            let operator_token = self.tokens.pop();
            let rhs_node = sub_grammar_rule(self)?;
            let operator_node =
                BinaryOperatorNode::try_new(expr_root_node, rhs_node, operator_token)
                    .unwrap_or_else(|e| panic!("{e}"));

            expr_root_node = ExpressionSubtreeRootNode::BinaryOperator(Box::new(operator_node))
        }

        Ok(expr_root_node)
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::parsing::ast::IdentifierNode;
    use crate::test::{DUMMY_TOKEN_BOOL, DUMMY_TOKEN_EOF, DUMMY_TOKEN_IDENTIFIER, DUMMY_TOKEN_INT};

    #[test]
    fn test_handle_expr_atom_rule_correctly_parses() -> Result<(), SyntaxError> {
        assert_eq!(
            NackParser::new(vec![DUMMY_TOKEN_BOOL.clone()]).handle_expr_atom_rule()?,
            ExpressionSubtreeRootNode::BoolLiteral(
                BoolLiteralNode::try_from(DUMMY_TOKEN_BOOL.clone())
                    .unwrap_or_else(|e| panic!("{e}"))
            )
        );
        assert_eq!(
            NackParser::new(vec![DUMMY_TOKEN_INT.clone()]).handle_expr_atom_rule()?,
            ExpressionSubtreeRootNode::IntLiteral(
                IntLiteralNode::try_from(DUMMY_TOKEN_INT.clone()).unwrap_or_else(|e| panic!("{e}"))
            )
        );
        assert_eq!(
            NackParser::new(vec![DUMMY_TOKEN_IDENTIFIER.clone()]).handle_expr_atom_rule()?,
            ExpressionSubtreeRootNode::Identifier(
                IdentifierNode::try_from(DUMMY_TOKEN_IDENTIFIER.clone())
                    .unwrap_or_else(|e| panic!("{e}"))
            )
        );

        Ok(())
    }

    #[test]
    fn test_handle_expr_atom_rule_returns_error_for_malformed_token_stream() {
        assert!(
            NackParser::new(vec![DUMMY_TOKEN_EOF.clone()])
                .handle_expr_atom_rule()
                .is_err()
        );
    }

    #[test]
    fn test_handle_expression_rule_correctly_parses() -> Result<(), SyntaxError> {
        assert_eq!(
            NackParser::new(vec![DUMMY_TOKEN_IDENTIFIER.clone()]).handle_expression_rule()?,
            ExpressionNode {
                subtree_node: ExpressionSubtreeRootNode::Identifier(
                    IdentifierNode::try_from(DUMMY_TOKEN_IDENTIFIER.clone())
                        .unwrap_or_else(|e| panic!("{e}"))
                )
            }
        );

        Ok(())
    }

    #[test]
    fn test_handle_expression_rule_returns_error_for_malformed_token_stream() {
        assert!(
            NackParser::new(vec![DUMMY_TOKEN_EOF.clone()])
                .handle_expression_rule()
                .is_err()
        );
    }

    #[test]
    fn test_handle_program_unit_rule_correctly_parses() -> Result<(), SyntaxError> {
        todo!("Program units not well defined yet")
    }

    #[test]
    fn test_handle_program_unit_rule_returns_error_for_malformed_token_stream() {
        todo!("Program units not well defined yet")
    }

    #[test]
    fn test_handle_program_rule_correctly_parses() -> Result<(), SyntaxError> {
        todo!("Program units not well defined yet")
    }

    #[test]
    fn test_handle_program_rule_returns_error_for_malformed_token_stream() {
        todo!("Program units not well defined yet")
    }
}
