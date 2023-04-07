use std::{process, io::Stdin, io::Read};
use rlox_scan::{scanner::Scanner, token::Token, token::TokenType};

struct Lox {
    had_error: bool
}

impl Lox{
    fn new() -> Self {
        Self {
            had_error: false
        }
    }

    fn run_file(&self, path: &str) {
        let mut file: std::fs::File = std::fs::File::open(path).unwrap_or_else(|err| {
            println!("Problem opening file: {err}");
            process::exit(1);
        });
    
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer);
        Self::run(buffer);
    }
    
    fn run_prompt(&self) {
        let stdin: Stdin = std::io::stdin();
        let mut buffer: String = String::new();
    
        loop {
            print!("> ");
            // Read line will borrow buffer and place the read line into it.
            match stdin.read_line(&mut buffer) {
                Ok(n) => {
                    // Run will take ownership of buffer.
                    Self::run(buffer)
                }
                Err(error) => {
                    println!("Error reading from repl...");
                    process::exit(1);
                }
            }
            // Buffer has been moved into run, so we need to create a new one.
            buffer = String::new();
        }
    }
    
    fn run(source: String) {
        // Scanner will take ownership of source.
        let scanner = Scanner::new(source);        
    
        for token in scanner.tokens {
            println!("{token}");
        }
    }
    
    fn error(&mut self, line: i32, message: String) {
        Self::report(self, line, "", &message);
    }
    
    fn report(&mut self, line: i32, is_where: &str, message: &str){
        eprintln!("[line {line}] Error {is_where}: {message}");
        self.had_error = true;
    }
}