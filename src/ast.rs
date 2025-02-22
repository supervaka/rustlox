use crate::{
    token::{Token, TokenType},
    types::Number,
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

#[derive(Debug, Clone, PartialEq)]
pub enum LitVal {
    Number(Number),
    String(String),
    True,
    False,
    Nil,
    NotExist,
}
impl<'a> std::fmt::Display for LitVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

enum BinaryOp {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Minus,
    Plus,
    Star,
    Slash,
}

enum UnaryOp {
    Bang,
    Minus,
}

impl Expr {
    fn evaluate(&self) -> Expr {
        match self {
            Expr::Binary { left, op, right } => todo!(),
            Expr::Grouping { expression } => todo!(),
            Expr::Literal(value) => todo!(),
            Expr::Unary { op, right } => todo!(),
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
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

    fn primary(&mut self) -> Option<Expr> {
        if self.match_(&[TokenType::False]) {
            return Some(Expr::Literal(LitVal::False));
        }
        if self.match_(&[TokenType::True]) {
            return Some(Expr::Literal(LitVal::True));
        }
        if self.match_(&[TokenType::Nil]) {
            return Some(Expr::Literal(LitVal::Nil));
        }

        if self.match_(&[TokenType::Number, TokenType::String]) {
            return Some(Expr::Literal(LitVal::False));
        }

        if self.match_(&[TokenType::LeftParen]) {
            return Some(Expr::Literal(LitVal::False));
        }

        None
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
        self.peek().value == *t
    }

    fn is_at_end(&self) -> bool {
        self.peek().value == TokenType::Eof
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
    use super::*;

    #[test]
    fn a() {
        let _x = Expr::Binary {
            left: Box::new(Expr::Unary {
                op: Token {
                    value: TokenType::Minus,
                    text: "-".to_string(),
                    line: 1,
                    literal: todo!(),
                },
                right: Box::new(Expr::Literal(LitVal::Number(123.0))),
            }),
            op: Token {
                value: TokenType::Star,
                text: "*".to_string(),
                line: 1,
                literal: todo!(),
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal(LitVal::Number(45.67))),
            }),
        };
        assert!(false);
    }
}
