#[derive(Debug)]
pub struct Token {
    position: SourcePosition,
    value: String,
    kind: TokenKind,
}

pub fn tokenize_source_string(source_string: &str) -> Vec<Token> {
    vec![]
}

#[derive(Debug)]
struct SourcePosition {
    line: u64,
    column: u64,
}

#[derive(Debug)]
enum TokenKind {
    Eof,
    Identifier,
    Ignore,
}
