use crate::lexing::{Token, TokenStream};

/// This enum represents the different types of AST nodes as well as the metadata associated with the types.
pub enum ASTNodeType {
    /// A node that represents a logical grouping of other nodes as its children. Grouping nodes have a string label.
    Grouping(String),
    /// A node that represents a Nack language token.
    Token(Token),
}

/// This struct represents a single node in a Nack AST. Nodes can be either grouping nodes or token nodes.
pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub children: Vec<ASTNode>,
}

/// This struct provides a parser used to turn a series of Nack language tokens into an AST.
///
/// Example
/// ```rust
/// let tokens = vec![];
/// let parser = NackParser::new(tokens);
/// let ast_root = parser.parse();
/// ```
pub struct NackParser {
    /// The token stream being parsed.
    tokens: TokenStream,
}

impl NackParser {
    /// Creates a new parser prepared to parse the given list of tokens.
    pub fn new(tokens: Vec<Token>) -> NackParser {
        NackParser { tokens: TokenStream::new(tokens) }
    }

    /// Parses the stored token stream and produces an AST. The returned node is the root of the AST.
    pub fn parse(self) -> ASTNode {
        todo!()
    }
}