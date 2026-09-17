use crate::lexing::{SourcePosition, Token, TokenKind};
use std::sync::LazyLock;

pub static DUMMY_TOKEN_BOOL: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("true"),
    kind: TokenKind::Identifier,
});
pub static DUMMY_TOKEN_INT: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("123"),
    kind: TokenKind::IntLiteral,
});
pub static DUMMY_TOKEN_IDENTIFIER: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("foo"),
    kind: TokenKind::Identifier,
});
pub static DUMMY_TOKEN_EOF: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::new(),
    kind: TokenKind::Eof,
});
pub static DUMMY_TOKEN_OPEN_PAREN: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("("),
    kind: TokenKind::LeftParen,
});
pub static DUMMY_TOKEN_CLOSE_PAREN: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from(")"),
    kind: TokenKind::RightParen,
});
pub static DUMMY_TOKEN_PLUS_SIGN: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("+"),
    kind: TokenKind::PlusSign,
});
pub static DUMMY_TOKEN_MULT_SIGN: LazyLock<Token> = LazyLock::new(|| Token {
    position: SourcePosition { line: 0, column: 0 },
    value: String::from("*"),
    kind: TokenKind::Asterisk,
});

#[macro_export]
macro_rules! create_dummy_subtree_node {
    ($subtree_enum_name:ident, $node_name:ident, $token:expr) => {
        ExpressionSubtreeRootNode::$subtree_enum_name(
            $node_name::try_from($token.clone()).unwrap_or_else(|e| panic!("{e}")),
        )
    };
}
