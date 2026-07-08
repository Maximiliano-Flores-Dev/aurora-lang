#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Assign,
    StringLiteral(String),
    Fn,
    OpenBrace,
    CloseBrace,
    Print,
    OpenParen,
    CloseParen,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() { return Token::EOF; }

        let ch = self.input[self.pos];
        match ch {
            '=' => { self.pos += 1; Token::Assign }
            '{' => { self.pos += 1; Token::OpenBrace }
            '}' => { self.pos += 1; Token::CloseBrace }
            '(' => { self.pos += 1; Token::OpenParen }
            ')' => { self.pos += 1; Token::CloseParen }
            '"' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < self.input.len() && self.input[self.pos] != '"' {
                    self.pos += 1;
                }
                let val: String = self.input[start..self.pos].iter().collect();
                self.pos += 1;
                Token::StringLiteral(val)
            }
            _ if ch.is_alphabetic() => {
                let start = self.pos;
                while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
                    self.pos += 1;
                }
                let ident: String = self.input[start..self.pos].iter().collect();
                match ident.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    "print" => Token::Print,
                    _ => Token::Identifier(ident),
                }
            }
            _ => { self.pos += 1; self.next_token() }
        }
    }
}
