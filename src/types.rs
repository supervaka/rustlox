use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

pub type Number = f64;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LitVal {
    Number(Number),
    String(String),
    Bool(bool),
    Nil,
    NotExist,
}

impl fmt::Display for LitVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitVal::Number(n) => write!(f, "{}", n),
            LitVal::String(s) => write!(f, "{}", s),
            LitVal::Bool(b) => write!(f, "{}", b),
            LitVal::Nil => write!(f, "nil"),
            LitVal::NotExist => write!(f, "not exist"),
        }
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
