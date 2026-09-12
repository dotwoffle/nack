use crate::lexing::{Token, TokenKind};

#[derive(Debug)]
pub enum ProgramUnitNode {
    Expression(ExpressionNode),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionSubtreeRootNode {
    IntLiteral(IntLiteralNode),
    BoolLiteral(BoolLiteralNode),
    Identifier(IdentifierNode),
}

#[derive(Debug)]
pub struct NackProgramAST {
    pub program_units: Vec<ProgramUnitNode>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionNode {
    pub subtree_node: ExpressionSubtreeRootNode,
}

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
