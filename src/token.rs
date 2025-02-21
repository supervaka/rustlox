use crate::types::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
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
    Identifier(String),
    String(String),
    Number(Number),

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
    pub value: TokenValue,
    pub text: String,
    pub line: usize,
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenValue::LeftParen => f.write_str("("),
            TokenValue::RightParen => f.write_str(")"),
            TokenValue::LeftBrace => f.write_str("{"),
            TokenValue::RightBrace => f.write_str("}"),
            TokenValue::Colon => f.write_str(":"),
            TokenValue::Comma => f.write_str(","),
            TokenValue::Dot => f.write_str("."),
            TokenValue::Minus => f.write_str("-"),
            TokenValue::Plus => f.write_str("+"),
            TokenValue::Question => f.write_str("?"),
            TokenValue::Semicolon => f.write_str(";"),
            TokenValue::Slash => f.write_str("/"),
            TokenValue::Star => f.write_str("*"),
            TokenValue::Bang => f.write_str("!"),
            TokenValue::BangEqual => f.write_str("!="),
            TokenValue::Equal => f.write_str("="),
            TokenValue::EqualEqual => f.write_str("=="),
            TokenValue::Greater => f.write_str(">"),
            TokenValue::GreaterEqual => f.write_str(">="),
            TokenValue::Less => f.write_str("<"),
            TokenValue::LessEqual => f.write_str("<="),
            TokenValue::Identifier(s) => f.write_str(s),
            TokenValue::String(s) => s.fmt(f),
            TokenValue::Number(n) => n.fmt(f),
            TokenValue::And => f.write_str("and"),
            TokenValue::Break => f.write_str("break"),
            TokenValue::Class => f.write_str("class"),
            TokenValue::Else => f.write_str("else"),
            TokenValue::False => f.write_str("false"),
            TokenValue::Fun => f.write_str("fun"),
            TokenValue::For => f.write_str("for"),
            TokenValue::If => f.write_str("if"),
            TokenValue::Nil => f.write_str("nil"),
            TokenValue::Or => f.write_str("or"),
            TokenValue::Print => f.write_str("print"),
            TokenValue::Return => f.write_str("return"),
            TokenValue::Super => f.write_str("super"),
            TokenValue::This => f.write_str("this"),
            TokenValue::True => f.write_str("true"),
            TokenValue::Var => f.write_str("var"),
            TokenValue::While => f.write_str("while"),
            TokenValue::Eof => f.write_str("\\d"),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            TokenValue::LeftParen => write!(f, "LEFT_PAREN {} null", self.value),
            TokenValue::RightParen => write!(f, "RIGHT_PAREN {} null", self.value),
            TokenValue::LeftBrace => write!(f, "LEFT_BRACE {} null", self.value),
            TokenValue::RightBrace => write!(f, "RIGHT_BRACE {} null", self.value),
            TokenValue::Comma => write!(f, "COMMA {} null", self.value),
            TokenValue::Dot => write!(f, "DOT {} null", self.value),
            TokenValue::Minus => write!(f, "MINUS {} null", self.value),
            TokenValue::Plus => write!(f, "PLUS {} null", self.value),
            TokenValue::Semicolon => write!(f, "SEMICOLON {} null", self.value),
            TokenValue::Slash => write!(f, "SLASH {} null", self.value),
            TokenValue::Star => write!(f, "STAR {} null", self.value),
            TokenValue::Bang => write!(f, "BANG {} null", self.value),
            TokenValue::BangEqual => write!(f, "BANG_EQUAL {} null", self.value),
            TokenValue::Equal => write!(f, "EQUAL {} null", self.value),
            TokenValue::EqualEqual => write!(f, "EQUAL_EQUAL {} null", self.value),
            TokenValue::Greater => write!(f, "GREATER {} null", self.value),
            TokenValue::GreaterEqual => write!(f, "GREATER_EQUAL {} null", self.value),
            TokenValue::Less => write!(f, "LESS {} null", self.value),
            TokenValue::LessEqual => write!(f, "LESS_EQUAL {} null", self.value),
            TokenValue::Identifier(_) => write!(f, "IDENTIFIER {} null", self.value),

            TokenValue::String(_) => write!(f, "STRING \"{}\" {}", self.value, self.value),
            TokenValue::Number(n) => {
                if n == n.floor() {
                    write!(f, "NUMBER {} {}.0", n, n)
                } else {
                    write!(f, "NUMBER {} {}", n, n)
                }
            }
            TokenValue::And => write!(f, "AND {} null", self.value),
            TokenValue::Class => write!(f, "CLASS {} null", self.value),
            TokenValue::Else => write!(f, "ELSE {} null", self.value),
            TokenValue::False => write!(f, "FALSE {} null", self.value),
            TokenValue::Fun => write!(f, "FUN {} null", self.value),
            TokenValue::For => write!(f, "FOR {} null", self.value),
            TokenValue::If => write!(f, "IF {} null", self.value),
            TokenValue::Nil => write!(f, "NIL {} null", self.value),
            TokenValue::Or => write!(f, "OR {} null", self.value),
            TokenValue::Print => write!(f, "PRINT {} null", self.value),
            TokenValue::Return => write!(f, "RETURN {} null", self.value),
            TokenValue::Super => write!(f, "SUPER {} null", self.value),
            TokenValue::This => write!(f, "THIS {} null", self.value),
            TokenValue::True => write!(f, "TRUE {} null", self.value),
            TokenValue::Var => write!(f, "VAR {} null", self.value),
            TokenValue::While => write!(f, "WHILE {} null", self.value),
            TokenValue::Colon => write!(f, "COLON {} null", self.value),
            TokenValue::Question => write!(f, "QUESTION {} null", self.value),
            TokenValue::Break => write!(f, "BREAK {} null", self.value),
            TokenValue::Eof => write!(f, "EOF  null"),
        }
    }
}
