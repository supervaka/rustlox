use crate::token::{Token, TokenType};
use crate::{types, Lox};
use types::LitVal;

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
            type_: TokenType::Eof,
            lexeme: String::new(),
            literal: LitVal::Nil,
            line: self.line,
        });
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token_default(TokenType::LeftParen),
            ')' => self.add_token_default(TokenType::RightParen),
            '{' => self.add_token_default(TokenType::LeftBrace),
            '}' => self.add_token_default(TokenType::RightBrace),
            ',' => self.add_token_default(TokenType::Comma),
            '.' => self.add_token_default(TokenType::Dot),
            '-' => self.add_token_default(TokenType::Minus),
            '+' => self.add_token_default(TokenType::Plus),
            ';' => self.add_token_default(TokenType::Semicolon),
            '*' => self.add_token_default(TokenType::Star),

            '!' => {
                if self.match_('=') {
                    self.add_token_default(TokenType::BangEqual);
                } else {
                    self.add_token_default(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token_default(TokenType::EqualEqual);
                } else {
                    self.add_token_default(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token_default(TokenType::LessEqual);
                } else {
                    self.add_token_default(TokenType::Less);
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token_default(TokenType::GreaterEqual);
                } else {
                    self.add_token_default(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_default(TokenType::Slash);
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
                    Lox::error(self.line, "Unexpected character.");
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
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token, LitVal::String(text));
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
        self.add_token(TokenType::String, LitVal::String(value));
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
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
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
        self.add_token(TokenType::Number, LitVal::Number(value));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token: TokenType, literal: LitVal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            type_: token,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    fn add_token_default(&mut self, token: TokenType) {
        self.add_token(token, LitVal::NotExist);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    /// chap04_scanning tests
    fn scanner() {
        let mut scanner = Scanner::new(
            "andy formless fo _ _123 _abc ab123
abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_"
                .to_string(),
        );
        let tokens = scanner.scan_tokens();

        // For now, just print the tokens.
        let expected: Vec<_> = "IDENTIFIER andy null
IDENTIFIER formless null
IDENTIFIER fo null
IDENTIFIER _ null
IDENTIFIER _123 null
IDENTIFIER _abc null
IDENTIFIER ab123 null
IDENTIFIER abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_ null
EOF  null"
            .split('\n')
            .collect();

        for (token, e) in zip(tokens, expected) {
            assert_eq!(&token.to_string(), e);
        }
    }
}
