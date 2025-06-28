use crate::{
    stmt::Stmt,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_end() {
            stmts.push(self.statement());
        }

        stmts
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().is_tt(&TokenType::EOF)
    }

    fn check_tt(&self, tt: &TokenType) -> bool {
        if !self.is_end() {
            false
        } else {
            self.peek().is_tt(tt)
        }
    }

    fn matches(&mut self, tts: &[TokenType]) -> bool {
        for tt in tts {
            if self.peek().is_tt(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, tt: TokenType) -> &Token {
        if self.check_tt(&tt) {
            return self.advance();
        }
        panic!("expected token type: {:?}", tt);
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(&[TokenType::Star, TokenType::Star]) {
            Stmt::Header(HeaderStmt {})
        } else {
            Stmt::Paragraph(ParagraphStmt {})
        }
    }
}
