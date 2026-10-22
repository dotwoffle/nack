use crate::lexing::{Token, TokenKind};
use crate::parsing::ast::BinaryOperator::{
    Addition, Division, LogicalAnd, LogicalOr, Multiplication, Subtraction,
};
use crate::parsing::ast::UnaryOperator::LogicalNot;
use crate::parsing::{AND_KEYWORD, NOT_KEYWORD, OR_KEYWORD};
use std::fmt::{Debug, Formatter};

/// This enum represents an AST node containing a program unit subtree.
#[derive(Debug, PartialEq)]
pub enum ProgramUnitNode {
    /// A value definition node.
    ValDef(ValDefNode),
}

impl ProgramUnitNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramUnitNode::ValDef(val_def_node) => val_def_node.dump(indent, f),
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
    /// A binary operator node. Currently must be a box due to recursive typing.
    BinaryOperator(Box<BinaryOperatorNode>),
    /// A unary operator node.
    UnaryOperator(Box<UnaryOperatorNode>),
}

impl ExpressionSubtreeRootNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionSubtreeRootNode::IntLiteral(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::BoolLiteral(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::Identifier(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::BinaryOperator(node) => node.dump(indent, f),
            ExpressionSubtreeRootNode::UnaryOperator(node) => node.dump(indent, f),
        }
    }
}

/// This enum represents all the possible binary operators.
#[derive(PartialEq)]
pub enum BinaryOperator {
    /// The addition operator.
    Addition,
    /// The subtraction operator.
    Subtraction,
    /// The multiplication operator.
    Multiplication,
    /// The division operator.
    Division,
    /// The logical AND operator.
    LogicalAnd,
    /// The logical OR operator.
    LogicalOr,
}

impl Debug for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Addition => "+",
                Subtraction => "-",
                Multiplication => "*",
                Division => "/",
                LogicalAnd => AND_KEYWORD,
                LogicalOr => OR_KEYWORD,
            }
        )
    }
}

impl From<&Token> for BinaryOperator {
    /// Constructs the appropriate BinaryOperator variant from the given token. The token's kind is
    /// used to determine which variant to construct. The mapping is as follows:
    ///
    /// - `Asterisk` -> `Multiplication`
    /// - `MinusSign` -> `Subtraction`
    /// - `PlusSign` -> `Addition`
    /// - `Slash` -> `Division`
    /// - `Identifier` -> Only valid if the token's value is a valid binary operator keyword. The
    ///   valid keywords are as follows:
    ///     - "and" -> `LogicalAnd`
    ///     - "or" -> `LogicalOr`
    ///
    /// If a token that doesn't match the above criteria is given, this function panics.
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Asterisk => Multiplication,
            TokenKind::MinusSign => Subtraction,
            TokenKind::PlusSign => Addition,
            TokenKind::Slash => Division,
            TokenKind::Identifier => match token.value.as_ref() {
                AND_KEYWORD => LogicalAnd,
                OR_KEYWORD => LogicalOr,
                _ => panic!("{} is not a valid binary operator keyword", token.value),
            },
            _ => panic!(
                "{:?} is not a valid token type for binary operators",
                token.kind
            ),
        }
    }
}

/// This enum represents all the possible unary operators.
#[derive(PartialEq)]
pub enum UnaryOperator {
    /// The logical NOT operator.
    LogicalNot,
}

impl Debug for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LogicalNot => NOT_KEYWORD,
            }
        )
    }
}

impl From<&Token> for UnaryOperator {
    /// Constructs the appropriate UnaryOperator variant from the given token. The token's kind is
    /// used to determine which variant to construct. The mapping is as follows:
    ///
    /// - `Identifier` -> Only valid if the token's value is a valid unary operator keyword. The
    ///   valid keywords are as follows:
    ///     - "not" -> `LogicalNot`
    ///
    /// If a token that doesn't match the above criteria is given, this function panics.
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Identifier => match token.value.as_ref() {
                NOT_KEYWORD => LogicalNot,
                _ => panic!("{} is not a valid unary operator keyword", token.value),
            },
            _ => panic!(
                "{:?} is not a valid token type for unary operators",
                token.kind
            ),
        }
    }
}

/// This struct represents the root node of a Nack AST.
#[derive(PartialEq)]
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

/// This struct represents an AST node that is the root of a binary operation expression subtree.
#[derive(Debug, PartialEq)]
pub struct BinaryOperatorNode {
    /// The root node of the left hand side of the binary expression.
    pub lhs: ExpressionSubtreeRootNode,
    /// The root node of the right hand side of the binary expression.
    pub rhs: ExpressionSubtreeRootNode,
    /// The binary operation this node represents.
    pub operator: BinaryOperator,
}

impl BinaryOperatorNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{:?}", "  ".repeat(indent), self.operator)?;
        self.lhs.dump(indent + 1, f)?;
        self.rhs.dump(indent + 1, f)
    }
}

/// This struct represents an AST node that is the root of a binary operation expression subtree.
#[derive(Debug, PartialEq)]
pub struct UnaryOperatorNode {
    /// The root node of the expression being operated on.
    pub expression: ExpressionSubtreeRootNode,
    /// The unary operation this node represents.
    pub operator: UnaryOperator,
}

impl UnaryOperatorNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{:?}", "  ".repeat(indent), self.operator)?;
        self.expression.dump(indent + 1, f)
    }
}

#[derive(Debug, PartialEq)]
pub struct ValDefNode {
    pub name: String,
    pub expression: ExpressionNode,
}

impl ValDefNode {
    fn dump(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{:?}", "  ".repeat(indent), self.name)?;
        self.expression.dump(indent, f)
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
