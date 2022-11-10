use crate::token_types::{Token, TokenType};
use std::ops::Not;

#[derive(Debug)]
pub enum Constant {
    None,
    Bool(bool),
    Str(String),
    Float(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Invert,
    Not,
}

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        calle: Box<Expr>,
        paren: Token,
        arguments: Box<Vec<Expr>>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Constant,
    },
    UnaryOp {
        operator: Token,
        operand: Box<Expr>,
    },
    Error {
        msg: String,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
pub struct ParserError {
    msg: String,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr{
        return self.expression()
    }

    fn expression(&mut self) -> Expr{
        self.equality()
    }

    //equality -> comparison (("!=", "==") comparison)*
    fn equality(&mut self)-> Expr{
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    //unary -> ("!" | "-") unary | primary;
    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::UnaryOp {
                operator,
                operand: Box::new(right),
            };
        }
        self.primary()
    }

    //factor -> expr (("/" | "*") expr)*;
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    //term -> expr (("+" | "-") expr)*;
    fn term(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    //comparison -> term ((">" | ">=" | "<")term)*
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(vec![
            TokenType::GREATER,
            TokenType::GreaterEqual,
            TokenType::LESS,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    //primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::FALSE]) {
            return Expr::Literal {
                value: Constant::Bool(false),
            };
        }
        if self.match_token(vec![TokenType::TRUE]) {
            return Expr::Literal {
                value: Constant::Bool(true),
            };
        }
        if self.match_token(vec![TokenType::NIL]) {
            return Expr::Literal {
                value: Constant::None,
            };
        }

        if self.match_token(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal {
                value: Constant::Str(self.previous().literal.as_ref().unwrap().clone()), //TODO: Clone or Copy ?
            };
        }
        return Expr::Error {
            msg: "Expect expression".to_string(),
        };
    }

    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
    fn advance(&mut self) -> &Token {
        if self.is_at_end().not() {
            self.current += 1;
        }
        return self.previous();
    }
}
