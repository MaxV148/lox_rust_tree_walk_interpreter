use crate::token_types::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            line: 1,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // one char token
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            //two char tokens
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            //slash commant
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '"' => {
                self.string();
            }
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifiert();
                } else {
                    eprintln!("Unexpected character at: {}", self.line);
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        *self.source.get(self.current - 1).unwrap()
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, expexted: char) -> bool {
        if self.is_end() {
            false
        } else if *self.source.get(self.current).unwrap() != expexted {
            false
        } else {
            self.current += 1;
            true
        }
    }
    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            *self.source.get(self.current).unwrap()
        }
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
            
        }
        if self.is_end() {
            eprintln!("Unterminated String at: {}", self.line);
        }
        //closing "
        self.advance();
        //Trim quotes
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_literal(TokenType::STRING, Some(value));
    }

    fn identifiert(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        //if text is  in keywords its a keyword otherwise an identifier
        let keyword = match text.as_str() {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        };
        self.add_token(keyword);
    }

    fn number(&mut self) {
        //consume number literal
        while self.peek().is_digit(10) {
            self.advance();
        }
        // look for fraction part
        if self.peek() == '.' && self.peek_mext().is_digit(10) {
            self.advance();
        }
        //consume rest of the number
        while self.peek().is_digit(10) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();

        self.add_token_with_literal(TokenType::NUMBER, Some(text));
    }

    fn peek_mext(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            *self.source.get(self.current + 1).unwrap()
        }
    }
}
