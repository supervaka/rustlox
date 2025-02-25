use crate::{token::Token, types::LitVal};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(LitVal),
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Variable(Token),
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
                LitVal::Bool(b) => b.to_string(),
                LitVal::Nil => "nil".to_string(),
                LitVal::NotExist => todo!(),
                LitVal::Function(lox_function) => todo!(),
                LitVal::Clock(clock) => todo!(),
            },
            Expr::Unary { op, right } => {
                format!("({} {})", op.type_, right.stringify())
            }
            Expr::Variable(token) => todo!(),
            Expr::Assign { name, value } => todo!(),
            Expr::Logical { left, op, right } => todo!(),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
        }
    }
}
