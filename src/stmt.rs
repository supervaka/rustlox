use expr::Expr;

use crate::{expr, token::Token};

pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Var {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
