use core::error;

use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
    types::LitVal,
    Lox,
};

#[derive(Debug, Clone)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_(&[TokenType::For]) {
            return self.for_stmt();
        }
        if self.match_(&[TokenType::If]) {
            return self.if_stmt();
        }
        if self.match_(&[TokenType::Print]) {
            return self.print_stmt();
        }
        if self.match_(&[TokenType::While]) {
            return self.while_stmt();
        }
        if self.match_(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expr_stmt()
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_(&[TokenType::Semicolon]) {
            None
        } else if self.match_(&[TokenType::Var]) {
            Some(self.var_decl()?)
        } else {
            Some(self.expr_stmt()?)
        };

        let mut condition = if !self.check(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(LitVal::Nil)
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            self.expression()?
        } else {
            Expr::Literal(LitVal::Nil)
        };
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if increment != Expr::Literal(LitVal::Nil) {
            body = Stmt::Block(vec![body, Stmt::Expr(increment)]);
        }

        if condition == Expr::Literal(LitVal::Nil) {
            condition = Expr::Literal(LitVal::Bool(true));
        }
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };
        if let Some(initlzlr) = initializer {
            body = Stmt::Block(vec![initlzlr, body]);
        };

        Ok(body)
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let val = match self.expression() {
            Ok(v) => Ok(Stmt::Print(v)),
            Err(e) => return Err(e),
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        val
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.expression() {
            Ok(expr) => Ok(Stmt::Expr(expr)),
            Err(e) => return Err(e),
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        expr
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        use TokenType as tt;
        let name = self.consume(&tt::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(&tt::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut params = Vec::new();
        if !self.check(&tt::RightParen) {
            loop {
                if params.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters.");
                }
                params.push(self.consume(&tt::Identifier, "Expect parameter name.")?);

                if !self.match_(&[tt::Comma]) {
                    break;
                }
            }
        }
        self.consume(&tt::RightParen, "Expect ')' after parameters.")?;

        self.consume(&tt::LeftBrace, &format!("Expect '{{' before {kind} body."))?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.declaration_helper() {
            Ok(n) => Ok(n),
            Err(_) => {
                self.synchronize();
                Ok(Stmt::Expr(Expr::Literal(LitVal::Nil)))
            }
        }
    }

    fn declaration_helper(&mut self) -> Result<Stmt, ParseError> {
        if self.match_(&[TokenType::Fun]) {
            return self.function("function");
        }
        if self.match_(&[TokenType::Var]) {
            return self.var_decl();
        }
        self.statement()
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(LitVal::Nil)
        };

        let _ = self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Ok(Stmt::Var { name, initializer })
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(token) => {
                    let name = token;
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(self.error(equals, "Invalid assignment target."));
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                op: operator,
                right: Box::new(right?),
            })
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                op: operator,
                right: Box::new(right?),
            })
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                op: operator,
                right: Box::new(right?),
            })
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Expr::Unary {
                op: operator,
                right: Box::new(right?),
            });
        }

        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self.match_(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary();

        loop {
            if self.match_(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }

        expr
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_(&[TokenType::False]) {
            return Ok(Expr::Literal(LitVal::Bool(false)));
        }
        if self.match_(&[TokenType::True]) {
            return Ok(Expr::Literal(LitVal::Bool(true)));
        }
        if self.match_(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LitVal::Nil));
        }

        if self.match_(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal));
        }

        if self.match_(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression.");

            return Ok(Expr::Grouping {
                expression: Box::new(expr?),
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
        Lox::token_error(&token, message);
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
                    lexeme: "-".to_string(),
                    line: 1,
                    literal: LitVal::NotExist,
                },
                right: Box::new(Expr::Literal(LitVal::Number(123.0))),
            }),
            op: Token {
                type_: TokenType::Star,
                lexeme: "*".to_string(),
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
        let expr = parser.expression();

        assert_eq!(
            expr.unwrap().stringify(),
            "(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))"
        );
    }
}
