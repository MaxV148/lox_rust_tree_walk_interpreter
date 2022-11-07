use crate::token_types::{Literal, Token, TokenType};
use std::str::FromStr;

pub struct Scanner<I: Iterator<Item = char>> {
    chars: I,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    col: usize,
    start: usize,
    current_char: Option<char>,
    look_a_head_char: Option<char>,
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
            current_char: None,
            look_a_head_char: None,
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
            .push(Token::new(TokenType::EOF, None, None, self.line, self.col));
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
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BANG_EQUAL, Some("!=".to_string()), None);
                } else {
                    self.add_token(TokenType::BANG, Some("=".to_string()), None);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, Some("==".to_string()), None);
                } else {
                    self.add_token(TokenType::EQUAL, Some("=".to_string()), None);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LESS_EQUAL, Some("<=".to_string()), None);
                } else {
                    self.add_token(TokenType::LESS, Some("<".to_string()), None);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL, Some(">=".to_string()), None);
                } else {
                    self.add_token(TokenType::GREATER, Some(">".to_string()), None);
                }
            }
            //slash command
            '/' => {
                if self.match_next('/') {
                    loop {
                        match self.look_a_head_char {
                            Some(n) => {
                                if n == '\n' {
                                    break;
                                } else {
                                    self.advance();
                                }
                            }
                            None => break,
                        }
                    }
                } else {
                    self.add_token(TokenType::SLASH, Some("/".to_string()), None);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            '"' => {
                self.parse_string();
            }
            _ => {
                if c.is_digit(10) {
                    self.parse_number(c);
                } else if c.is_alphabetic() {
                    self.parse_identifier(c);
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
        literal: Option<Literal>,
    ) {
        //TODO: calculate Col
        let col = match &lexeme {
            Some(lex) => self.col - lex.len(),
            None => self.col,
        };

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line, col));
    }
    fn advance(&mut self) -> Option<char> {
        let mut current = self.look_a_head_char;
        //init
        if current.is_none() {
            current = self.chars.next();
        }
        let next = self.chars.next();
        self.current_char = current;
        self.look_a_head_char = next;
        self.col += 1;
        current
    }

    fn match_next(&mut self, expected: char) -> bool {
        match self.look_a_head_char {
            Some(c) => {
                if c == expected {
                    self.advance();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn parse_string(&mut self) {
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

        self.add_token(
            TokenType::STRING,
            None,
            Some(Literal::new(Some(new_string), None)),
        );
    }

    fn parse_number(&mut self, first_c: char) {
        let mut num_str = String::new();
        num_str.push(first_c);
        loop {
            match self.look_a_head_char {
                Some(look_a_head) => {
                    if look_a_head.is_digit(10) {
                        match self.advance() {
                            Some(next) => num_str.push(next),
                            None => break,
                        }
                    } else if look_a_head == '.' && !num_str.contains('.') {
                        match self.advance() {
                            Some(next) => num_str.push(next),
                            None => break,
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        let num = f64::from_str(num_str.as_str()).expect("Invalid decial literal");
        self.add_token(TokenType::NUMBER, None, Some(Literal::new(None, Some(num))));
    }

    fn parse_identifier(&mut self, first_c: char) {
        let mut ident = String::new();
        ident.push(first_c);
        loop {
            match self.look_a_head_char {
                Some(look_a_head) => {
                    if look_a_head.is_alphabetic() || look_a_head == '_' {
                        match self.advance() {
                            Some(next) => {
                                ident.push(next);
                                if self.match_keyword(&ident).is_some() {
                                    self.add_token(
                                        self.match_keyword(&ident).unwrap(),
                                        Some(ident),
                                        None,
                                    );
                                    return;
                                }
                            }
                            None => break,
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        self.add_token(TokenType::IDENTIFIER, Some(ident), None);
    }
    fn match_keyword(&self, string: &str) -> Option<TokenType> {
        match string {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None,
        }
    }
}
