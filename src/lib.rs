mod environment;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod types;

use core::fmt;
use std::io::Write;

use anyhow::{anyhow, Error, Result};
use interpreter::{Interpreter, RuntimeError};
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenType};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(&mut self, path: &str) -> Result<()> {
        let contents = std::fs::read_to_string(path).expect("file to be readable");
        self.run(contents);

        if unsafe { HAD_ERROR } {
            std::process::exit(65);
        }
        if unsafe { HAD_RUNTIME_ERROR } {
            std::process::exit(70);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut line = String::new();
            if std::io::stdin().read_line(&mut line)? > 0 {
                if let Err(e) = self.run(line) {
                    eprintln!("{}", e);
                    unsafe {
                        HAD_ERROR = true;
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().clone();

        let mut parser = Parser::new(tokens);
        let stmts = match parser.parse() {
            Ok(it) => it,
            Err(err) => return Err(anyhow!("parser.parse() error in lib.rs")),
        };
        let mut interpreter = Interpreter::new();
        interpreter.interpret(stmts);

        Ok(())
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    pub fn runtime_error(error: RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        unsafe {
            HAD_RUNTIME_ERROR = true;
        };
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.type_ == TokenType::Eof {
            Self::report(token.line, " at end", message)
        } else {
            let s = format!(" at '{}'", token.lexeme);
            Self::report(token.line, &s, message)
        }
    }

    fn report(line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error{location}: {message}");
        unsafe { HAD_ERROR = true }
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
