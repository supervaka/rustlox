use crate::{
    parser::{Expr, LitVal},
    token::{Token, TokenType},
};

struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, expr: Expr) -> LitVal {
        let value = self.evaluate(expr);
        value
    }

    fn evaluate(&mut self, expr: Expr) -> LitVal {
        match expr {
            Expr::Binary { left, op, right } => self.eval_binary(left, op, right),
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Literal(lit_val) => lit_val,
            Expr::Unary { op, right } => {
                let right = self.evaluate(*right);
                match right {
                    LitVal::Number(x) => match op.type_ {
                        TokenType::Minus => LitVal::Number(-x), // TODO: impl neg?
                        _ => panic!(),
                    },
                    _ => match op.type_ {
                        TokenType::Bang => LitVal::Bool(!is_truthy(right)),
                        _ => panic!(),
                    },
                }
            }
        }
    }

    fn eval_binary(
        &mut self,
        left: Box<Expr>,
        op: crate::token::Token,
        right: Box<Expr>,
    ) -> LitVal {
        let left = self.evaluate(*left);
        let right = self.evaluate(*right);

        match op.type_ {
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            TokenType::Greater => LitVal::Bool(left > right),
            TokenType::GreaterEqual => LitVal::Bool(left >= right),
            TokenType::Less => LitVal::Bool(left < right),
            TokenType::LessEqual => LitVal::Bool(left <= right),
            TokenType::BangEqual => LitVal::Bool(left != right),
            TokenType::EqualEqual => LitVal::Bool(left == right),
            _ => unreachable!(),
        }
    }
}

/// `false` and `nil` are falsey, and everything else is truthy
fn is_truthy(val: LitVal) -> bool {
    match val {
        LitVal::Bool(b) => b,
        LitVal::Nil => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    #[test]
    fn evaluate() {
        let mut scanner = Scanner::new("(5 - (3 - 1)) + -1".to_string());
        let tokens = scanner.scan_tokens().clone();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        let mut interpreter = Interpreter {};
        let val = interpreter.interpret(expr);
        assert_eq!(val.to_string(), "2");
    }
}
