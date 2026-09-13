use crate::lexing::{Token, TokenKind};

/// This enum represents an AST node containing a program unit subtree.
#[derive(Debug)]
pub enum ProgramUnitNode {
    /// An expression node.
    Expression(ExpressionNode),
}

/// This enum represents an AST expression subtree.
#[derive(Debug, PartialEq)]
pub enum ExpressionSubtreeRootNode {
    /// An integer literal node.
    IntLiteral(IntLiteralNode),
    /// A boolean literal node.
    BoolLiteral(BoolLiteralNode),
    /// An identifier node.
    Identifier(IdentifierNode),
}

/// This struct represents the root node of a Nack AST.
#[derive(Debug)]
pub struct NackProgramAST {
    /// Each program unit subtree in this program.
    pub program_units: Vec<ProgramUnitNode>,
}

/// This struct represents the root node of an expression tree.
#[derive(Debug, PartialEq)]
pub struct ExpressionNode {
    /// The expression subtree root node that this expression contains.
    pub subtree_node: ExpressionSubtreeRootNode,
}

/// Declares a struct that represents a single node of the AST, containing a token of a specific
/// kind. The declared struct implements `TryFrom<Token>` and only succeeds if the given token is
/// the required kind.
macro_rules! declare_token_node {
    ($name:ident, $kind:pat) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            token: Token,
        }

        impl $name {
            pub fn token(&self) -> &Token {
                &self.token
            }
        }

        impl TryFrom<Token> for $name {
            type Error = String;

            fn try_from(token: Token) -> Result<Self, Self::Error> {
                if matches!(token.kind, $kind) {
                    Ok($name { token })
                } else {
                    Err(format!(
                        concat!(
                            "Cannot construct a ",
                            stringify!($name),
                            " node from a {:?} token"
                        ),
                        token.kind
                    ))
                }
            }
        }
    };
}

declare_token_node!(IntLiteralNode, TokenKind::IntLiteral);
declare_token_node!(BoolLiteralNode, TokenKind::Identifier);
declare_token_node!(IdentifierNode, TokenKind::Identifier);
