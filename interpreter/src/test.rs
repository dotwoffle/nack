use crate::lexing::{SourcePosition, Token, TokenKind};
use std::sync::LazyLock;

pub static DUMMY_TOKEN_BOOL: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::Identifier, "true"));
pub static DUMMY_TOKEN_INT: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::IntLiteral, "123"));
pub static DUMMY_TOKEN_IDENTIFIER: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::Identifier, "foo"));
pub static DUMMY_TOKEN_EOF: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::Eof, ""));
pub static DUMMY_TOKEN_OPEN_PAREN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::LeftParen, "("));
pub static DUMMY_TOKEN_CLOSE_PAREN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::RightParen, ")"));
pub static DUMMY_TOKEN_PLUS_SIGN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::PlusSign, "+"));
pub static DUMMY_TOKEN_MINUS_SIGN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::MinusSign, "-"));
pub static DUMMY_TOKEN_MULT_SIGN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::Asterisk, "*"));
pub static DUMMY_TOKEN_DIVIDE_SIGN: LazyLock<Token> =
    LazyLock::new(|| create_dummy_token(TokenKind::Slash, "/"));

#[macro_export]
macro_rules! create_dummy_subtree_node {
    ($subtree_enum_name:ident, $node_name:ident, $token:expr) => {
        ExpressionSubtreeRootNode::$subtree_enum_name(
            $node_name::try_from($token.clone()).unwrap_or_else(|e| panic!("{e}")),
        )
    };
}

pub fn create_dummy_token(token_kind: TokenKind, value: &str) -> Token {
    Token {
        position: SourcePosition { line: 0, column: 0 },
        value: String::from(value),
        kind: token_kind,
    }
}
