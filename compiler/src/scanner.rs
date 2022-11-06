use crate::token_types::{Token, TokenType};

pub struct Scanner<I: Iterator<Item = char>> {
    chars: I,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    col: usize,
    start: usize,
}
impl<I> Scanner<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Self {
        Scanner {
            chars,
            tokens: Vec::new(),
            current: 0,
            line: 1,
            col: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        loop {
            match self.advance() {
                Some(c) => {
                    self.start = self.current;
                    self.scan_token(c);
                }
                None => break,
            };
        }

        self.tokens
            .push(Token::new(TokenType::EOF, None, None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self, c: char) {
        match c {
            // one char token
            '(' => self.add_token(TokenType::LeftParen, Some(c.to_string()), None),
            ')' => self.add_token(TokenType::RightParen, Some(c.to_string()), None),
            '{' => self.add_token(TokenType::RightBrace, Some(c.to_string()), None),
            '}' => self.add_token(TokenType::LeftBrace, Some(c.to_string()), None),
            ',' => self.add_token(TokenType::COMMA, Some(c.to_string()), None),
            '.' => self.add_token(TokenType::DOT, Some(c.to_string()), None),
            '-' => self.add_token(TokenType::MINUS, Some(c.to_string()), None),
            '+' => self.add_token(TokenType::PLUS, Some(c.to_string()), None),
            ';' => self.add_token(TokenType::SEMICOLON, Some(c.to_string()), None),
            '*' => self.add_token(TokenType::STAR, Some(c.to_string()), None),
            //two char tokens
            '!' => {}
            '=' => {}
            '<' => {}
            '>' => {}
            //slash commant
            '/' => {}
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '"' => {
                self.string();
            }
            _ => {
                if c.is_digit(10) {
                    //self.number();
                } else if c.is_alphabetic() {
                    self.identifier(c);
                } else {
                    eprintln!("Unexpected character at: {}", self.line);
                }
            }
        }
    }

    fn add_token(
        &mut self,
        token_type: TokenType,
        lexeme: Option<String>,
        literal: Option<String>,
    ) {
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
    fn advance(&mut self) -> Option<char> {
        self.col += 1;
        self.chars.next()
    }

    fn string(&mut self) {
        let mut new_string = String::new();
        loop {
            match self.advance() {
                Some(next) => {
                    if next != '"' {
                        new_string.push(next);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        self.add_token(TokenType::STRING, None, Some(new_string));
    }
    fn identifier(&mut self, first_c: char) {
        let mut ident = String::new();
        ident.push(first_c);
        loop {
            match self.advance() {
                Some(next) => {
                    if next.is_alphanumeric() {
                        ident.push(next);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        self.add_token(TokenType::IDENTIFIER, Some(ident), None);
    }
}
