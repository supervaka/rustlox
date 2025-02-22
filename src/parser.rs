use core::fmt;

use crate::{
    token::{Token, TokenType},
    types::Number,
    Lox,
};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(LitVal),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn stringify(&self) -> String {
        match self {
            Expr::Binary { left, op, right } => {
                format!("({} {} {})", op.type_, left.stringify(), right.stringify())
            }
            Expr::Grouping { expression } => format!("(group {})", expression.stringify()),
            Expr::Literal(lit_val) => match lit_val {
                LitVal::Number(n) => {
                    if *n == n.floor() {
                        format!("{}.0", n)
                    } else {
                        format!("{}", n)
                    }
                }
                LitVal::String(s) => s.to_string(),
                LitVal::True => "true".to_string(),
                LitVal::False => "false".to_string(),
                LitVal::Nil => "nil".to_string(),
                LitVal::NotExist => todo!(),
            },
            Expr::Unary { op, right } => {
                format!("({} {})", op.type_, right.stringify())
            }
        }
    }
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum LitVal {
    Number(Number),
    String(String),
    True,
    False,
    Nil,
    NotExist,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                op: operator,
                right: Box::new(right),
            };
        }

        self.primary().unwrap()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&[TokenType::False]) {
            return Ok(Expr::Literal(LitVal::False));
        }
        if self.match_(&[TokenType::True]) {
            return Ok(Expr::Literal(LitVal::True));
        }
        if self.match_(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LitVal::Nil));
        }

        if self.match_(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal));
        }

        if self.match_(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression.");

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn consume(&mut self, t: &TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(t) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        Lox::parse_error(token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();
        use TokenType::*;

        while !self.is_at_end() {
            if self.previous().type_ == TokenType::Semicolon {
                return;
            }

            match self.peek().type_ {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn match_(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().type_ == *t
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    use super::*;

    #[test]
    fn ast() {
        let x = Expr::Binary {
            left: Box::new(Expr::Unary {
                op: Token {
                    type_: TokenType::Minus,
                    text: "-".to_string(),
                    line: 1,
                    literal: LitVal::NotExist,
                },
                right: Box::new(Expr::Literal(LitVal::Number(123.0))),
            }),
            op: Token {
                type_: TokenType::Star,
                text: "*".to_string(),
                line: 1,
                literal: LitVal::NotExist,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal(LitVal::Number(45.67))),
            }),
        };
        assert_eq!(x.stringify(), "(* (- 123.0) (group 45.67))");
    }

    #[test]
    fn parse() {
        let mut scanner = Scanner::new("(5 - (3 - 1)) + -1".to_string());
        let tokens = scanner.scan_tokens().clone();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert_eq!(
            expr.stringify(),
            "(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))"
        );
    }
}
