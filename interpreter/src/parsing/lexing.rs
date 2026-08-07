use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

/// This struct represents a single Nack language token, parsed from a source string. Tokens have a
/// type, a position, and a string value, representing the token as it appeared within the source string.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The position within the source string where the first character of this token is found.
    position: SourcePosition,
    /// The string value of this token.
    value: String,
    /// The type of token this is.
    kind: TokenKind,
}

/// Turns a source string into a series of Nack language tokens. Tokens with type "Ignore" are not
/// included in the output, and a single "Eof" token is always appended to the end of the token list.
///
/// # Example
/// ```rust
/// let tokens = tokenize_source_string("let x = 5");
/// println!("{tokens:?");
/// ```
pub fn tokenize_source_string(source_string: &str) -> Vec<Token> {
    vec![]
}

/// This struct indicates the position within a source string at which a specific token is found. By
/// convention, line and column numbers both start at 1.
#[derive(Debug, PartialEq)]
struct SourcePosition {
    /// The line number.
    line: u64,
    /// The column number.
    column: u64,
}

/// This enum represents the different kinds of Nack language tokens.
#[derive(Debug, Eq, Hash, PartialEq)]
enum TokenKind {
    /// A special token type indicating the end of the token stream.
    Eof,
    /// Identifiers and keywords.
    Identifier,
    /// Any amount of contiguous whitespace.
    Ignore,
}

/// Maps token types to regex patterns that match tokens of that type.
static TOKEN_PATTERNS: LazyLock<HashMap<TokenKind, Regex>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    m.insert(
        TokenKind::Identifier,
        Regex::new(r"[_a-zA-Z][_a-zA-Z0-9]*").expect("Failed to compile regex for Identifier"),
    );
    m.insert(
        TokenKind::Ignore,
        Regex::new(r"\s+").expect("Failed to compile regex for Ignore"),
    );

    m
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_regex_matches_correct_strings() -> Result<(), &'static str> {
        let identifier_regex = TOKEN_PATTERNS
            .get(&TokenKind::Identifier)
            .ok_or("No pattern defined in TOKEN_PATTERNS for Identifier")?;

        assert!(regex_matches_entire_string(identifier_regex, "identifier"));
        assert!(regex_matches_entire_string(identifier_regex, "some_value"));
        assert!(regex_matches_entire_string(identifier_regex, "someValue"));
        assert!(regex_matches_entire_string(identifier_regex, "SomeValue"));
        assert!(regex_matches_entire_string(identifier_regex, "sOmE_vAlUe"));
        assert!(regex_matches_entire_string(
            identifier_regex,
            "_private_value"
        ));
        assert!(regex_matches_entire_string(identifier_regex, "_"));
        assert!(regex_matches_entire_string(identifier_regex, "x"));
        assert!(regex_matches_entire_string(identifier_regex, "value1"));
        assert!(regex_matches_entire_string(identifier_regex, "____"));
        assert!(regex_matches_entire_string(
            identifier_regex,
            "a_1_R_5234234___"
        ));

        assert!(!regex_matches_entire_string(identifier_regex, ""));
        assert!(!regex_matches_entire_string(identifier_regex, "\n"));
        assert!(!regex_matches_entire_string(identifier_regex, "some value"));
        assert!(!regex_matches_entire_string(identifier_regex, "1value"));
        assert!(!regex_matches_entire_string(identifier_regex, "!value"));
        assert!(!regex_matches_entire_string(identifier_regex, "value?"));
        assert!(!regex_matches_entire_string(identifier_regex, "|value|"));

        Ok(())
    }

    #[test]
    fn test_ignore_regex_matches_correct_strings() -> Result<(), &'static str> {
        let ignore_regex = TOKEN_PATTERNS
            .get(&TokenKind::Ignore)
            .ok_or("No pattern defined in TOKEN_PATTERNS for Ignore")?;

        assert!(regex_matches_entire_string(ignore_regex, " "));
        assert!(regex_matches_entire_string(ignore_regex, "     "));
        assert!(regex_matches_entire_string(ignore_regex, "\t"));
        assert!(regex_matches_entire_string(ignore_regex, "\n"));
        assert!(regex_matches_entire_string(ignore_regex, "\n\n\n"));
        assert!(regex_matches_entire_string(ignore_regex, " \t\n"));
        assert!(regex_matches_entire_string(ignore_regex, "   \n   \t"));

        assert!(!regex_matches_entire_string(ignore_regex, ""));
        assert!(!regex_matches_entire_string(ignore_regex, " foo"));
        assert!(!regex_matches_entire_string(ignore_regex, "\nfoo"));
        assert!(!regex_matches_entire_string(ignore_regex, "\tfoo"));
        assert!(!regex_matches_entire_string(ignore_regex, "foo "));
        assert!(!regex_matches_entire_string(ignore_regex, "foo\n"));
        assert!(!regex_matches_entire_string(ignore_regex, "foo\t"));

        Ok(())
    }

    #[test]
    fn test_valid_source_strings_are_tokenized_correctly() {
        let eof_value = "".to_owned();

        assert_eq!(tokenize_source_string(""), vec![Token {
            position: SourcePosition { line: 1, column: 1 },
            value: eof_value.clone(),
            kind: TokenKind::Eof
        }]);
        assert_eq!(tokenize_source_string("  \n  "), vec![Token {
            position: SourcePosition { line: 2, column: 3 },
            value: eof_value.clone(),
            kind: TokenKind::Eof
        }]);
        assert_eq!(tokenize_source_string("test"), vec![
            Token {
                position: SourcePosition { line: 1, column: 1 },
                value: "test".to_owned(),
                kind: TokenKind::Identifier
            },
            Token {
                position: SourcePosition { line: 1, column: 3 },
                value: eof_value.clone(),
                kind: TokenKind::Eof
            }
        ]);
        assert_eq!(tokenize_source_string("big_thing split   \n across multiple \n\nlines\n"), vec![
            Token {
                position: SourcePosition { line: 1, column: 1 },
                value: "big_thing".to_owned(),
                kind: TokenKind::Identifier
            },
            Token {
                position: SourcePosition {
                    line: 1,
                    column: 11
                },
                value: "split".to_owned(),
                kind: TokenKind::Identifier
            },
            Token {
                position: SourcePosition { line: 2, column: 9 },
                value: "multiple".to_owned(),
                kind: TokenKind::Identifier
            },
            Token {
                position: SourcePosition { line: 4, column: 1 },
                value: "lines".to_owned(),
                kind: TokenKind::Identifier
            },
            Token {
                position: SourcePosition { line: 5, column: 1 },
                value: eof_value.clone(),
                kind: TokenKind::Eof
            }
        ]);
    }

    fn regex_matches_entire_string(pattern: &Regex, string: &str) -> bool {
        pattern
            .find(string)
            .is_some_and(|m| m.start() == 0 && m.end() == string.len())
    }
}
