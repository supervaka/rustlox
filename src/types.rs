use core::fmt;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::stmt::Stmt;
use crate::token::Token;

pub type Number = f64;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LitVal>,
    ) -> Result<LitVal, RuntimeError>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LitVal {
    Number(Number),
    String(String),
    Bool(bool),
    Nil,
    NotExist,
    Function(LoxFunction),
    Clock(Clock),
}

impl fmt::Display for LitVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitVal::Number(n) => write!(f, "{}", n),
            LitVal::String(s) => write!(f, "{}", s),
            LitVal::Bool(b) => write!(f, "{}", b),
            LitVal::Nil => write!(f, "nil"),
            LitVal::NotExist => write!(f, "not exist"),
            LitVal::Clock(_) => write!(f, "<native fn>"),
            LitVal::Function(lox_function) => write!(f, "<fn>"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Clock;

impl LoxCallable for Clock {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LitVal>,
    ) -> Result<LitVal, RuntimeError> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(LitVal::Number(since_the_epoch.as_secs_f64()))
    }

    fn arity(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LoxFunction {
    decl: Rc<Stmt>,
}

impl LoxFunction {
    pub fn new(decl: Rc<Stmt>) -> Self {
        LoxFunction { decl }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        if let Stmt::Function {
            name: _,
            ref params,
            body: _,
        } = *self.decl
        {
            params.len()
        } else {
            unreachable!("self.decl should always be a function");
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LitVal>,
    ) -> Result<LitVal, RuntimeError> {
        let environment = Rc::new(RefCell::new(Environment::new_with_enclosing(
            interpreter.globals.clone(),
        )));

        if let Stmt::Function {
            name: _,
            ref params,
            ref body,
        } = *self.decl
        {
            for i in 0..params.len() {
                environment
                    .borrow_mut()
                    .define(params[i].lexeme.clone(), arguments[i].clone());
            }
            interpreter.exec_block(body, environment)?;
        } else {
            unreachable!("self.decl should always be a function");
        }
        Ok(LitVal::Nil)
    }
}

impl Sub for LitVal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (LitVal::Number(a), LitVal::Number(b)) => LitVal::Number(a - b),
            _ => panic!("Subtraction is only supported for numbers"),
        }
    }
}
impl Div for LitVal {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (LitVal::Number(a), LitVal::Number(b)) => LitVal::Number(a / b),
            _ => panic!("Division is only supported for numbers"),
        }
    }
}

impl Mul for LitVal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (LitVal::Number(a), LitVal::Number(b)) => LitVal::Number(a * b),
            _ => panic!("Multiplication is only supported for numbers"),
        }
    }
}

impl Add for LitVal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (LitVal::Number(a), LitVal::Number(b)) => LitVal::Number(a + b),
            (LitVal::String(a), LitVal::String(b)) => LitVal::String(a + &b),
            _ => panic!("Addition is only supported for numbers and strings"),
        }
    }
}
