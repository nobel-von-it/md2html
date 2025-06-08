#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    text: String,
    literal: Option<String>,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Text,

    Star,
    Underscore,
    Hash,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    EOF,
}

impl Token {
    pub fn new(token_type: TokenType, text: String, literal: Option<String>, line: usize) -> Token {
        Token {
            token_type,
            text,
            literal,
            line,
        }
    }
}

