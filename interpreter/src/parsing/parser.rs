use crate::lexing::{SyntaxError, Token, TokenKind, TokenStream};
use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};

/// This enum represents the different types of AST nodes as well as the metadata associated with the types.
#[derive(PartialEq)]
pub enum ASTNodeType {
    /// A node that represents a logical grouping of other nodes as its children. Grouping nodes have a string label.
    Grouping(String),
    /// A node that represents a Nack language token.
    Token(Token),
}

/// This struct represents a single node in a Nack AST. Nodes can be either grouping nodes or token nodes.
pub struct ASTNode {
    /// The type of node this is.
    pub node_type: ASTNodeType,
    /// All child nodes of this node.
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    /// Creates a grouping node with the given label and children.
    fn of_grouping(label: String, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Grouping(label),
            children,
        }
    }

    /// Creates a token node with the given token and children.
    fn of_token(token: Token, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Token(token),
            children,
        }
    }

    /// Prints this node's debug string representation, then all of its children indented below it.
    fn dump(&self, indent: usize, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}{}",
            "  ".repeat(indent),
            match &self.node_type {
                ASTNodeType::Grouping(label) => label.clone(),
                ASTNodeType::Token(token) => format!("{:?} (\"{}\")", token.kind, token.value),
            }
        )?;
        self.dump(indent + 1, fmt)
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_type == other.node_type && self.children == other.children
    }
}

impl Debug for ASTNode {
    /// Dumps the tree to a human-readable string.
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        self.dump(0, fmt)
    }
}

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
    /// Creates a new parser prepared to parse the given list of tokens.
    pub fn new(tokens: Vec<Token>) -> NackParser {
        NackParser {
            tokens: TokenStream::new(tokens),
        }
    }

    /// Parses the stored token stream and produces an AST. The returned node is the root of the AST.
    pub fn parse(mut self) -> Result<ASTNode, SyntaxError> {
        self.handle_program_rule()
    }

    fn handle_program_rule(&mut self) -> Result<ASTNode, SyntaxError> {
        let mut children = vec![];

        while !self.tokens.next_token_has_types(&[TokenKind::Eof]) {
            children.push(self.handle_program_unit_rule()?);
        }

        Ok(ASTNode::of_grouping(PROGRAM_LABEL.to_owned(), children))
    }

    fn handle_program_unit_rule(&mut self) -> Result<ASTNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier | TokenKind::IntLiteral => self.handle_expression_rule(),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }

    fn handle_expression_rule(&mut self) -> Result<ASTNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier | TokenKind::IntLiteral => self.handle_expr_atom_rule(),
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }

    fn handle_expr_atom_rule(&mut self) -> Result<ASTNode, SyntaxError> {
        match self.tokens.peek(0).kind {
            TokenKind::Identifier | TokenKind::IntLiteral => {
                Ok(ASTNode::of_token(self.tokens.pop(), vec![]))
            }
            _ => Err(SyntaxError {
                position: self.tokens.peek(0).position,
                message: String::from("Expected an expression here"),
            }),
        }
    }
}

pub static PROGRAM_LABEL: &str = "PROGRAM";

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::lexing::SourcePosition;

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
}
