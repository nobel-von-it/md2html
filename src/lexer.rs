use crate::token::{Token, TokenType};

pub struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,

    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        self.tokens.clone()
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_text(&self, c: char) -> bool {
        self.is_alphanum(c) || "-_=+!@#$%&()".contains(c)
    }

    fn is_alphanum(&self, c: char) -> bool {
        c.is_alphanumeric()
    }

    fn advance(&mut self) -> char {
        // skdljf *slkdfjl*
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1]
    }

    fn add_token_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn text(&mut self) {
        let mut p = self.peek();
        while self.is_text(p) && !self.is_end() {
            self.advance();
            p = self.peek();
        }

        if self.is_end() {
            panic!("infinity text")
        }

        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        self.add_token(TokenType::Text, Some(text));
    }

    // fn bold(&mut self) {
    //     while !(self.peek() == '*' || self.peek_next() == '*') && !self.is_end() {
    //         self.advance();
    //     }
    //
    //     if self.is_end() {
    //         panic!("infinity bold")
    //     }
    //
    //     self.advance();
    //     let text = self.source[self.start..self.current]
    //         .iter()
    //         .collect::<String>();
    //     self.add_token(TokenType::Bold, Some(text));
    // }
    //
    // fn cursive(&mut self) {
    //     while self.peek() != '*' && !self.is_end() {
    //         self.advance();
    //     }
    //
    //     if self.is_end() {
    //         panic!("infinity cursive")
    //     }
    //
    //     self.advance();
    //     let text = self.source[self.start + 1..self.current - 1]
    //         .iter()
    //         .collect::<String>();
    //     self.add_token(TokenType::Cursive, Some(text));
    // }
    //
    // fn header(&mut self) {
    //     let mut header_size = 1;
    //     while self.peek() == '#' && !self.is_end() {
    //         header_size += 1;
    //         self.advance();
    //     }
    //
    //     while self.peek() != '\n' && !self.is_end() {
    //         let c = self.advance();
    //         match c {
    //             '*' if self.peek() == '*' => self.bold(),
    //             '*' => self.cursive(),
    //             _ => {}
    //         }
    //     }
    //
    //     if self.is_end() {
    //         panic!("infinity header")
    //     }
    //
    //     let text = self.source[self.start..self.current]
    //         .iter()
    //         .collect::<String>();
    //     self.add_token(TokenType::H(header_size), Some(text));
    // }
    //
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '#' => self.add_token_type(TokenType::Hash),
            '*' => self.add_token_type(TokenType::Star),
            '_' => self.add_token_type(TokenType::Underscore),
            '(' => self.add_token_type(TokenType::LeftParen),
            ')' => self.add_token_type(TokenType::RightParen),
            '[' => self.add_token_type(TokenType::LeftBracket),
            ']' => self.add_token_type(TokenType::RightBracket),
            '{' => self.add_token_type(TokenType::LeftBrace),
            '}' => self.add_token_type(TokenType::RightBrace),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c if self.is_alphanum(c) => self.text(),
            _ => {}
        }
    }

    fn debug(&self) {
        println!("start: {}", self.start);
        println!("current: {}", self.current);
        println!("line: {}", self.line);
    }
}
