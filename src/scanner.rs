use crate::token::{Token, TokenValue};
use crate::types::Number;
use crate::Lox;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            value: TokenValue::Eof,
            text: String::new(),
            line: self.line,
        });
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenValue::LeftParen),
            ')' => self.add_token(TokenValue::RightParen),
            '{' => self.add_token(TokenValue::LeftBrace),
            '}' => self.add_token(TokenValue::RightBrace),
            ',' => self.add_token(TokenValue::Comma),
            '.' => self.add_token(TokenValue::Dot),
            '-' => self.add_token(TokenValue::Minus),
            '+' => self.add_token(TokenValue::Plus),
            ';' => self.add_token(TokenValue::Semicolon),
            '*' => self.add_token(TokenValue::Star),

            '!' => {
                if self.match_('=') {
                    self.add_token(TokenValue::BangEqual);
                } else {
                    self.add_token(TokenValue::Bang);
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token(TokenValue::EqualEqual);
                } else {
                    self.add_token(TokenValue::Equal);
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token(TokenValue::LessEqual);
                } else {
                    self.add_token(TokenValue::Less);
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token(TokenValue::GreaterEqual);
                } else {
                    self.add_token(TokenValue::Greater);
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenValue::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),

            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    Lox::error(self.line, "Unexpected character");
                }
            }
        };
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token = match text.as_str() {
            "and" => TokenValue::And,
            "class" => TokenValue::Class,
            "else" => TokenValue::Else,
            "false" => TokenValue::False,
            "fun" => TokenValue::Fun,
            "for" => TokenValue::For,
            "if" => TokenValue::If,
            "nil" => TokenValue::Nil,
            "or" => TokenValue::Or,
            "print" => TokenValue::Print,
            "return" => TokenValue::Return,
            "super" => TokenValue::Super,
            "this" => TokenValue::This,
            "true" => TokenValue::True,
            "var" => TokenValue::Var,
            "while" => TokenValue::While,
            _ => TokenValue::Identifier(text),
        };

        self.add_token(token);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token(TokenValue::String(value));
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || c.is_ascii_digit();
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenValue::Number(value));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token: TokenValue) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            value: token,
            text,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
