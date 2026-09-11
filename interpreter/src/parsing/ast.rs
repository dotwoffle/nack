use crate::lexing::{Token, TokenKind};

#[derive(Debug)]
pub enum ProgramUnitNode {
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExpressionSubtreeRootNode {
    IntLiteral(IntLiteralNode),
    BoolLiteral(BoolLiteralNode),
}

#[derive(Debug)]
pub struct NackProgramAST {
    pub program_units: Vec<ProgramUnitNode>,
}

#[derive(Debug)]
pub struct ExpressionNode {
    pub subtree_node: ExpressionSubtreeRootNode,
}

macro_rules! declare_literal_node {
    ($name:ident, $kind:pat) => {
        #[derive(Debug)]
        pub struct $name {
            pub token: Token,
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

declare_literal_node!(IntLiteralNode, TokenKind::IntLiteral);
declare_literal_node!(BoolLiteralNode, TokenKind::Identifier);
