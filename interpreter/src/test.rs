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
