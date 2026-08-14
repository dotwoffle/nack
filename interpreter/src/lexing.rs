use regex::Regex;
use std::collections::VecDeque;
use std::{collections::HashMap, sync::LazyLock};

/// This enum represents the different kinds of Nack language tokens.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenKind {
    /// A special token type indicating the end of the token stream.
    Eof,
    /// Identifiers and keywords.
    Identifier,
    /// Any amount of contiguous whitespace.
    Ignore,
}

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

/// This struct represents a syntax error encountered during parsing.
#[derive(Debug)]
pub struct SyntaxError {
    /// The character position in the file at which the syntax error occurred.
    pub position: SourcePosition,
    /// A human-readable error message.
    pub message: String,
}

/// This struct provides a wrapper around a list of Nack language tokens, turning it into a one-way consuming stream.
pub struct TokenStream {
    /// The backing list of tokens for this stream.
    tokens: VecDeque<Token>,
}

impl TokenStream {
    /// Creates a new token stream from the given list of tokens.
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream {
            tokens: VecDeque::from(tokens),
        }
    }

    /// Removes and returns the next token in the stream. If there are no tokens left, this function panics.
    pub fn pop(&mut self) -> Token {
        self.tokens
            .pop_front()
            .expect("Tried to pop from an empty token stream")
    }

    /// Checks that the next token in the stream has the specified token type, then pops it. If the next token does not
    /// have the required type, an error is returned containing the caller-provided error message. If there are no
    /// tokens left in the stream, this function panics.
    pub fn require_and_pop(
        &mut self,
        required_kind: &TokenKind,
        error_message: String,
    ) -> Result<Token, String> {
        (self.peek(0).kind != *required_kind)
            .then(|| self.pop())
            .ok_or(error_message)
    }

    /// Returns a view of the token in the stream that is `lookahead` positions ahead of the current stream position. If
    /// there are not enough tokens left in the stream to get the one at the requested position, this function panics.
    pub fn peek(&self, lookahead: usize) -> &Token {
        &self.tokens[lookahead]
    }

    /// Checks if the next token in the stream is any of the given types. If there are no tokens left in the stream,
    /// this function panics.
    pub fn next_token_has_types(&self, types: &[TokenKind]) -> bool {
        types.contains(&self.peek(0).kind)
    }
}

/// This struct indicates the position within a source string at which a specific token is found. By
/// convention, line and column numbers both start at 1.
#[derive(Clone, Debug, PartialEq)]
pub struct SourcePosition {
    /// The line number.
    pub line: u64,
    /// The column number.
    pub column: u64,
}

impl SourcePosition {
    /// Updates this position by examining a token value extracted from the source string, consuming the old position.
    ///
    /// Example
    /// ```rust
    /// let pos = SourcePosition {line: 1, column: 1};
    /// assert_eq!(pos.update_from_extracted_token("test\nhi"), SourcePosition {line: 2, column: 2};
    /// ```
    fn update_from_extracted_token(self, token_value: &str) -> SourcePosition {
        let num_newlines = token_value.chars().filter(|c| *c == '\n').count();

        if num_newlines == 0 {
            SourcePosition {
                line: self.line,
                column: self.column + token_value.len() as u64,
            }
        } else {
            SourcePosition {
                line: self.line + num_newlines as u64,
                column: (token_value.len()
                    - token_value
                        .rfind('\n')
                        .expect("Newline count in extracted token was not 0"))
                    as u64,
            }
        }
    }
}

/// Turns a source string into a series of Nack language tokens. Tokens with type "Ignore" are not
/// included in the output, and a single "Eof" token is always appended to the end of the token list.
///
/// # Example
/// ```rust
/// let tokens = tokenize_source_string("let x = 5");
/// println!("{tokens:?");
/// ```
pub fn tokenize_source_string(source_string: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = vec![];
    let mut current_position = SourcePosition { line: 1, column: 1 };
    let mut current_source_string = source_string;

    while !current_source_string.is_empty() {
        let (token_kind, token_value) =
            find_next_token(current_source_string).ok_or(SyntaxError {
                position: current_position.clone(),
                message: String::from("No tokens matched here"),
            })?;

        if token_kind != TokenKind::Ignore {
            tokens.push(Token {
                position: current_position.clone(),
                value: token_value.to_owned(),
                kind: token_kind,
            });
        }

        current_source_string = &current_source_string[token_value.len()..];
        current_position = current_position.update_from_extracted_token(token_value);
    }

    tokens.push(Token {
        position: current_position,
        value: String::from(""),
        kind: TokenKind::Eof,
    });

    Ok(tokens)
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

/// Determines the next token present in the source string, and returns a tuple containing the token type and the token value.
///
/// Example
/// ```rust
/// assert_eq!(find_next_token("hi"), (TokenKind::Identifier, "hi"));
/// ```
fn find_next_token(source_string: &str) -> Option<(TokenKind, &str)> {
    TOKEN_PATTERNS
        .iter()
        .map(|(token_type, pattern)| (token_type, pattern.find(source_string)))
        .filter_map(|(token_type, pattern)| pattern.map(|pattern| (token_type, pattern)))
        .filter(|(_, match_info)| match_info.start() == 0)
        .max_by(|match1, match2| match1.1.len().cmp(&match2.1.len()))
        .map(|possible_token| (*possible_token.0, possible_token.1.as_str()))
}

#[cfg(test)]
mod lexing_tests {
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
    fn test_valid_source_strings_are_tokenized_correctly() -> Result<(), SyntaxError> {
        let eof_value = "".to_owned();

        assert_eq!(
            tokenize_source_string("")?,
            vec![Token {
                position: SourcePosition { line: 1, column: 1 },
                value: eof_value.clone(),
                kind: TokenKind::Eof
            }]
        );
        assert_eq!(
            tokenize_source_string("  \n  ")?,
            vec![Token {
                position: SourcePosition { line: 2, column: 3 },
                value: eof_value.clone(),
                kind: TokenKind::Eof
            }]
        );
        assert_eq!(
            tokenize_source_string("test")?,
            vec![
                Token {
                    position: SourcePosition { line: 1, column: 1 },
                    value: "test".to_owned(),
                    kind: TokenKind::Identifier
                },
                Token {
                    position: SourcePosition { line: 1, column: 5 },
                    value: eof_value.clone(),
                    kind: TokenKind::Eof
                }
            ]
        );
        assert_eq!(
            tokenize_source_string("big_thing split   \n across multiple \n\nlines\n")?,
            vec![
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
                    position: SourcePosition { line: 2, column: 2 },
                    value: "across".to_owned(),
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
            ]
        );

        Ok(())
    }

    #[test]
    fn test_tokenizing_bad_source_string_returns_syntax_error() {
        assert_syntax_error_occurs("???", &SourcePosition { line: 1, column: 1 });
        assert_syntax_error_occurs(
            "valid stuff until ???",
            &SourcePosition {
                line: 1,
                column: 19,
            },
        );
        assert_syntax_error_occurs(
            "valid\nlines\nuntil\n???",
            &SourcePosition { line: 4, column: 1 },
        );
    }

    /// Checks that a given regex pattern matches an entire given string, not just part of it.
    fn regex_matches_entire_string(pattern: &Regex, string: &str) -> bool {
        pattern
            .find(string)
            .is_some_and(|m| m.start() == 0 && m.end() == string.len())
    }

    /// Asserts that a syntax error occurs at the given position in the source string.
    fn assert_syntax_error_occurs(bad_source_string: &str, bad_token_position: &SourcePosition) {
        let result = match tokenize_source_string(bad_source_string) {
            Ok(_) => panic!("Expected a syntax error to be returned"),
            Err(e) => e,
        };
        assert_eq!(result.position, *bad_token_position);
    }
}
