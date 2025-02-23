use crate::token::Token;
use crate::Lox;
use crate::{environment::Environment, expr::Expr, stmt::Stmt, token::TokenType, types::LitVal};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        RuntimeError {
            token,
            message: message.to_string(),
        }
    }
}

impl From<anyhow::Error> for RuntimeError {
    fn from(error: anyhow::Error) -> Self {
        RuntimeError {
            token: Token::default(), // may need to adjust this to provide a meaningful token
            message: error.to_string(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => Lox::runtime_error(e),
            };
        }
    }

    fn execute(&mut self, stmt: Stmt) -> Result<LitVal, RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => self.evaluate(expr),
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                Ok(value)
            }
            Stmt::Var { name, initializer } => {
                let value = if initializer != Expr::Literal(LitVal::Nil) {
                    self.evaluate(initializer)?
                } else {
                    LitVal::Nil
                };
                self.env.borrow_mut().define(name.lexeme, value.clone());
                Ok(value)
            }
            Stmt::Block(stmts) => self.exec_block(
                stmts,
                Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::clone(
                    &self.env,
                )))),
            ),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition)?) {
                    self.execute(*then_branch)
                } else if let Some(else_) = else_branch {
                    self.execute(*else_)
                } else {
                    Ok(LitVal::Nil)
                }
            }
            Stmt::While { condition, body } => todo!(),
        }
    }

    fn exec_block(
        &mut self,
        stmts: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<LitVal, RuntimeError> {
        let prev = Rc::clone(&self.env);
        self.env = env;
        for st in stmts {
            self.execute(st)?;
        }
        self.env = prev;
        Ok(LitVal::Nil)
    }

    fn evaluate(&mut self, expr: Expr) -> Result<LitVal, RuntimeError> {
        match expr {
            Expr::Binary { left, op, right } => self.eval_binary(*left, op, *right),
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Literal(lit_val) => Ok(lit_val),
            Expr::Unary { op, right } => {
                let right = self.evaluate(*right)?;
                match op.type_ {
                    TokenType::Bang => Ok(LitVal::Bool(!is_truthy(&right))),
                    TokenType::Minus => match right {
                        LitVal::Number(x) => Ok(LitVal::Number(-x)),
                        _ => Err(RuntimeError::new(op, "Operand must be a number.")),
                    },
                    _ => unreachable!("grammar should imply that this never happens"),
                }
            }
            Expr::Variable(token) => Ok(self.env.borrow().get(&token)?),
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.env.borrow_mut().assign(&name, &value)?;
                Ok(value)
            }
            Expr::Logical { left, op, right } => {
                let left = self.evaluate(*left)?;
                if op.type_ == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else if !is_truthy(&left) {
                    return Ok(left);
                }
                self.evaluate(*right)
            }
        }
    }

    fn eval_binary(&mut self, left: Expr, op: Token, right: Expr) -> Result<LitVal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        fn check_number_operands(
            operator: &Token,
            left: &LitVal,
            right: &LitVal,
        ) -> Result<(), RuntimeError> {
            if let (LitVal::Number(_), LitVal::Number(_)) = (left, right) {
                Ok(())
            } else {
                Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be numbers.",
                ))
            }
        }
        use TokenType::*;
        match op.type_ {
            Minus | Slash | Star | Greater | GreaterEqual | Less | LessEqual => {
                check_number_operands(&op, &left, &right)?
            }
            Plus => match (&left, &right) {
                (LitVal::Number(_), LitVal::Number(_)) => (),
                (LitVal::String(_), LitVal::String(_)) => (),
                _ => {
                    return Err(RuntimeError::new(
                        op,
                        "Operands must be two numbers or two strings.",
                    ))
                }
            },
            _ => (),
        };

        match op.type_ {
            Minus => Ok(left - right),
            Slash => Ok(left / right),
            Star => Ok(left * right),
            Plus => Ok(left + right),
            Greater => Ok(LitVal::Bool(left > right)),
            GreaterEqual => Ok(LitVal::Bool(left >= right)),
            Less => Ok(LitVal::Bool(left < right)),
            LessEqual => Ok(LitVal::Bool(left <= right)),
            BangEqual => Ok(LitVal::Bool(left != right)),
            EqualEqual => Ok(LitVal::Bool(left == right)),
            _ => unreachable!(),
        }
    }
}

/// `false` and `nil` are falsey, and everything else is truthy
fn is_truthy(val: &LitVal) -> bool {
    match val {
        LitVal::Bool(b) => *b,
        LitVal::Nil => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner, Lox};

    use super::*;

    #[test]
    fn evaluate() {
        fn f(s: &str, expected: &str) {
            let mut scanner = Scanner::new(s.to_string());
            let tokens = scanner.scan_tokens().clone();

            let mut parser = Parser::new(tokens);
            let expr = parser.expression();

            let mut interpreter = Interpreter::new();
            let val = interpreter.evaluate(expr.unwrap()).unwrap();
            assert_eq!(val.to_string(), expected);
        }
        f("(5 - (3 - 1)) + -1", "2");
        f("\"hello \" + \"world\"", "hello world");
        f("!1", &false.to_string());
        // f("a", "b");
    }

    #[test]
    fn interpret() {
        let mut lox = Lox::new();
        let _ = lox.run("print true;".to_string());
        let _ = lox.run("print \"one\";".to_string());
        let _ = lox.run("print 2 + 1;".to_string());
    }

    #[test]
    fn assignment() {
        let mut lox = Lox::new();
        let s = "var a = 1;
        print a = 2;"
            .to_string();
        let _ = lox.run(s);
    }
}
