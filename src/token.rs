use crate::{ast::LitVal, types::Number};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Colon,
    Question,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: TokenType,
    pub text: String,
    pub literal: LitVal,
    pub line: usize,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenType::LeftParen => f.write_str("("),
            TokenType::RightParen => f.write_str(")"),
            TokenType::LeftBrace => f.write_str("{"),
            TokenType::RightBrace => f.write_str("}"),
            TokenType::Colon => f.write_str(":"),
            TokenType::Comma => f.write_str(","),
            TokenType::Dot => f.write_str("."),
            TokenType::Minus => f.write_str("-"),
            TokenType::Plus => f.write_str("+"),
            TokenType::Question => f.write_str("?"),
            TokenType::Semicolon => f.write_str(";"),
            TokenType::Slash => f.write_str("/"),
            TokenType::Star => f.write_str("*"),
            TokenType::Bang => f.write_str("!"),
            TokenType::BangEqual => f.write_str("!="),
            TokenType::Equal => f.write_str("="),
            TokenType::EqualEqual => f.write_str("=="),
            TokenType::Greater => f.write_str(">"),
            TokenType::GreaterEqual => f.write_str(">="),
            TokenType::Less => f.write_str("<"),
            TokenType::LessEqual => f.write_str("<="),
            TokenType::Identifier => f.write_str(""),
            TokenType::String => f.write_str(""),
            TokenType::Number => f.write_str(""),
            TokenType::And => f.write_str("and"),
            TokenType::Break => f.write_str("break"),
            TokenType::Class => f.write_str("class"),
            TokenType::Else => f.write_str("else"),
            TokenType::False => f.write_str("false"),
            TokenType::Fun => f.write_str("fun"),
            TokenType::For => f.write_str("for"),
            TokenType::If => f.write_str("if"),
            TokenType::Nil => f.write_str("nil"),
            TokenType::Or => f.write_str("or"),
            TokenType::Print => f.write_str("print"),
            TokenType::Return => f.write_str("return"),
            TokenType::Super => f.write_str("super"),
            TokenType::This => f.write_str("this"),
            TokenType::True => f.write_str("true"),
            TokenType::Var => f.write_str("var"),
            TokenType::While => f.write_str("while"),
            TokenType::Eof => f.write_str("\\d"),
        }
    }
}

impl<'a> std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            TokenType::LeftParen => write!(f, "LEFT_PAREN {} null", self.value),
            TokenType::RightParen => write!(f, "RIGHT_PAREN {} null", self.value),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {} null", self.value),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE {} null", self.value),
            TokenType::Comma => write!(f, "COMMA {} null", self.value),
            TokenType::Dot => write!(f, "DOT {} null", self.value),
            TokenType::Minus => write!(f, "MINUS {} null", self.value),
            TokenType::Plus => write!(f, "PLUS {} null", self.value),
            TokenType::Semicolon => write!(f, "SEMICOLON {} null", self.value),
            TokenType::Slash => write!(f, "SLASH {} null", self.value),
            TokenType::Star => write!(f, "STAR {} null", self.value),
            TokenType::Bang => write!(f, "BANG {} null", self.value),
            TokenType::BangEqual => write!(f, "BANG_EQUAL {} null", self.value),
            TokenType::Equal => write!(f, "EQUAL {} null", self.value),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL {} null", self.value),
            TokenType::Greater => write!(f, "GREATER {} null", self.value),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL {} null", self.value),
            TokenType::Less => write!(f, "LESS {} null", self.value),
            TokenType::LessEqual => write!(f, "LESS_EQUAL {} null", self.value),
            TokenType::Identifier => write!(
                f,
                "IDENTIFIER {} null",
                match &self.literal {
                    LitVal::String(s) => s,
                    _ => "",
                }
            ),

            TokenType::String => {
                let s = match &self.literal {
                    LitVal::String(s) => s,
                    _ => panic!(),
                };
                write!(f, "STRING \"{}\" {}", s, s)
            }
            TokenType::Number => {
                let n = match &self.literal {
                    LitVal::Number(n) => n,
                    _ => panic!(""),
                };
                if *n == n.floor() {
                    write!(f, "NUMBER {} {}.0", n, n)
                } else {
                    write!(f, "NUMBER {} {}", n, n)
                }
            }
            TokenType::And => write!(f, "AND {} null", self.value),
            TokenType::Class => write!(f, "CLASS {} null", self.value),
            TokenType::Else => write!(f, "ELSE {} null", self.value),
            TokenType::False => write!(f, "FALSE {} null", self.value),
            TokenType::Fun => write!(f, "FUN {} null", self.value),
            TokenType::For => write!(f, "FOR {} null", self.value),
            TokenType::If => write!(f, "IF {} null", self.value),
            TokenType::Nil => write!(f, "NIL {} null", self.value),
            TokenType::Or => write!(f, "OR {} null", self.value),
            TokenType::Print => write!(f, "PRINT {} null", self.value),
            TokenType::Return => write!(f, "RETURN {} null", self.value),
            TokenType::Super => write!(f, "SUPER {} null", self.value),
            TokenType::This => write!(f, "THIS {} null", self.value),
            TokenType::True => write!(f, "TRUE {} null", self.value),
            TokenType::Var => write!(f, "VAR {} null", self.value),
            TokenType::While => write!(f, "WHILE {} null", self.value),
            TokenType::Colon => write!(f, "COLON {} null", self.value),
            TokenType::Question => write!(f, "QUESTION {} null", self.value),
            TokenType::Break => write!(f, "BREAK {} null", self.value),
            TokenType::Eof => write!(f, "EOF  null"),
        }
    }
}
