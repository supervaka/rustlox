use expr::Expr;

use crate::{expr, token::Token};

pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
}
