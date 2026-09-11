use crate::lexing::{Token, TokenKind};

pub enum ProgramUnitNode {
    Expression(ExpressionNode),
}

pub enum ExpressionSubtreeRootNode {
    SubExpression(Box<ExpressionNode>),
    IntLiteral(IntLiteralNode),
    BoolLiteral(BoolLiteralNode),
}

#[derive(Debug)]
pub struct NackProgramAST {
    pub program_units: Vec<ProgramUnitNode>,
}

pub struct ExpressionNode {
    subtree_node: ExpressionSubtreeRootNode,
}

impl ExpressionNode {
    pub fn new(subtree_node: ExpressionSubtreeRootNode) -> Self {
        ExpressionNode { subtree_node }
    }
}

macro_rules! declare_literal_node {
    ($name:ident, $kind:pat) => {
        pub struct $name {
            token: Token,
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
