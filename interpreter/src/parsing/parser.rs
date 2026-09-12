use crate::lexing::{SyntaxError, Token, TokenKind, TokenStream};
use crate::parsing::ast::{
    BoolLiteralNode, ExpressionNode, ExpressionSubtreeRootNode, IntLiteralNode, NackProgramAST,
    ProgramUnitNode,
};

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

    /// Parses the stored token stream and produces an AST. The returned node is the root of the
    /// AST.
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
        match self.tokens.peek(0).kind {
            TokenKind::Identifier | TokenKind::IntLiteral => self
                .handle_expr_atom_rule()
                .map(|subtree_node| ExpressionNode { subtree_node }),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }

    /// Parses the EXPR_ATOM language rule and returns the root of the produced subtree.
    fn handle_expr_atom_rule(&mut self) -> Result<ExpressionSubtreeRootNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier => Ok(ExpressionSubtreeRootNode::BoolLiteral(
                BoolLiteralNode::try_from(self.tokens.pop()).unwrap_or_else(|e| panic!("{e}")),
            )),
            TokenKind::IntLiteral => Ok(ExpressionSubtreeRootNode::IntLiteral(
                IntLiteralNode::try_from(self.tokens.pop()).unwrap_or_else(|e| panic!("{e}")),
            )),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::lexing::SourcePosition;
    use crate::parsing::EXPRESSION_LABEL;

    #[test]
    fn test_handle_expr_atom_rule_correctly_parses() -> Result<(), SyntaxError> {
        assert_eq!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::from("true"),
                kind: TokenKind::Identifier,
            }])
            .handle_expr_atom_rule()?,
            ASTNode::of_token(
                Token {
                    position: SourcePosition { line: 0, column: 0 },
                    value: String::from("true"),
                    kind: TokenKind::Identifier,
                },
                vec![]
            )
        );
        assert_eq!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::from("123"),
                kind: TokenKind::IntLiteral,
            }])
            .handle_expr_atom_rule()?,
            ASTNode::of_token(
                Token {
                    position: SourcePosition { line: 0, column: 0 },
                    value: String::from("123"),
                    kind: TokenKind::IntLiteral,
                },
                vec![]
            )
        );
        assert_eq!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::from("foo"),
                kind: TokenKind::Identifier,
            }])
            .handle_expr_atom_rule()?,
            ASTNode::of_token(
                Token {
                    position: SourcePosition { line: 0, column: 0 },
                    value: String::from("foo"),
                    kind: TokenKind::Identifier,
                },
                vec![]
            )
        );

        Ok(())
    }

    #[test]
    fn test_handle_expr_atom_rule_returns_error_for_malformed_token_stream() {
        assert!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::new(),
                kind: TokenKind::Eof
            }])
            .handle_expr_atom_rule()
            .is_err()
        );
    }

    #[test]
    fn test_handle_expression_rule_correctly_parses() -> Result<(), SyntaxError> {
        assert_eq!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::from("foo"),
                kind: TokenKind::Identifier,
            }])
            .handle_expression_rule()?,
            ASTNode::of_grouping(
                EXPRESSION_LABEL,
                vec![ASTNode::of_token(
                    Token {
                        position: SourcePosition { line: 0, column: 0 },
                        value: String::from("foo"),
                        kind: TokenKind::Identifier,
                    },
                    vec![]
                )]
            )
        );

        Ok(())
    }

    #[test]
    fn test_handle_expression_rule_returns_error_for_malformed_token_stream() {
        assert!(
            NackParser::new(vec![Token {
                position: SourcePosition { line: 0, column: 0 },
                value: String::new(),
                kind: TokenKind::Eof
            }])
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
