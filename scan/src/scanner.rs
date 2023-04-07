use crate::token::{Token, TokenType};

pub struct Scanner{
    source: String,
    pub tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&self) {
        while(!self.is_at_end()) {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::from(""), String::from(""), 0));
    }

    fn scan_token(&self){
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, String::from("(")),
            ')' => self.add_token(TokenType::RightParen, String::from(")")),
            '{' => self.add_token(TokenType::LeftBrace, String::from("{")),
            '}' => self.add_token(TokenType::RightBrace, String::from("}")),
            ',' => self.add_token(TokenType::Comma, String::from(",")),
            '.' => self.add_token(TokenType::Dot, String::from(".")),
            '-' => self.add_token(TokenType::Minus, String::from("-")),
            '+' => self.add_token(TokenType::Plus, String::from("+")),
            ';' => self.add_token(TokenType::Semicolon, String::from(";")),
            '*' => self.add_token(TokenType::Star, String::from("*")),
        }
    }

    fn add_token(&self, token_type: TokenType, lexeme: String) {
        let text: String = self.source.chars().skip(self.start as usize).take((self.current - self.start) as usize).collect();
        self.tokens.push(Token::new(token_type, lexeme, text, self.line));
    }

    fn advance(&self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count() as i32
    }
}