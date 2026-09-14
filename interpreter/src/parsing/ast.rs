use crate::lexing::{Token, TokenKind};
use std::fmt::{Debug, Formatter};

/// This enum represents an AST node containing a program unit subtree.
#[derive(Debug)]
pub enum ProgramUnitNode {
    /// An expression node.
    Expression(ExpressionNode),
}

impl ProgramUnitNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramUnitNode::Expression(expr_node) => expr_node.dump(indent, f),
        }
    }
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
    /// A binary operator node.
    BinaryOperator(Box<BinaryOperatorNode>),
}

impl ExpressionSubtreeRootNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionSubtreeRootNode::IntLiteral(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::BoolLiteral(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::Identifier(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::BinaryOperator(node) => node.dump(indent, f),
        }
    }
}

/// This struct represents the root node of a Nack AST.
pub struct NackProgramAST {
    /// Each program unit subtree in this program.
    pub program_units: Vec<ProgramUnitNode>,
}

impl Debug for NackProgramAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program")?;

        for program_unit in &self.program_units {
            program_unit.dump(1, f)?;
        }

        Ok(())
    }
}

/// This struct represents the root node of an expression tree.
#[derive(Debug, PartialEq)]
pub struct ExpressionNode {
    /// The expression subtree root node that this expression contains.
    pub subtree_node: ExpressionSubtreeRootNode,
}

impl ExpressionNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}Expression", "  ".repeat(indent))?;
        self.subtree_node.dump(indent + 1, f)
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperatorNode {
    pub lhs: ExpressionSubtreeRootNode,
    pub rhs: ExpressionSubtreeRootNode,
    token: Token,
}

impl BinaryOperatorNode {
    pub fn try_new(
        lhs: ExpressionSubtreeRootNode,
        rhs: ExpressionSubtreeRootNode,
        token: Token,
    ) -> Result<Self, String> {
        if matches!(
            token.kind,
            TokenKind::Asterisk | TokenKind::MinusSign | TokenKind::PlusSign | TokenKind::Slash
        ) {
            Ok(Self { lhs, rhs, token })
        } else {
            Err(format!(
                "Cannot construct a BinaryOperatorNode from a {:?} token",
                token.kind
            ))
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}", "  ".repeat(indent), self.token.value)?;
        self.lhs.dump(indent + 1, f)?;
        self.rhs.dump(indent + 1, f)
    }
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

            fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{}{}", "  ".repeat(indent), self.token.value)
            }
        }

        impl TryFrom<Token> for $name {
            type Error = String;

            fn try_from(token: Token) -> Result<Self, Self::Error> {
                if matches!(token.kind, $kind) {
                    Ok(Self { token })
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
