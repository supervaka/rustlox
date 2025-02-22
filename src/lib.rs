mod ast;
mod scanner;
mod token;
mod types;

use std::io::Write;

use anyhow::{bail, Result};
use scanner::Scanner;

static mut HAD_RUNTIME_ERROR: bool = false;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> Result<()> {
        let contents = std::fs::read_to_string(path).expect("file to be readable");

        self.run(contents)?;

        if self.had_error {
            std::process::exit(65);
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
                    self.had_error = false;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        // For now, just print the tokens.
        for token in tokens {
            println!("{}", token);
        }

        Ok(())
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error {location}: {message}");
        unsafe { HAD_RUNTIME_ERROR = true }
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
