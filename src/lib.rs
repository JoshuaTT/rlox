use std::{process, io::Stdin, io::Read};
use rlox_scan::{scanner::Scanner, token::Token};

fn run_file(path: &str) {
    let mut file: std::fs::File = std::fs::File::open(path).unwrap_or_else(|err| {
        println!("Problem opening file: {err}");
        process::exit(1);
    });

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer);
    run(buffer);
}

fn run_prompt() {
    let stdin: Stdin = std::io::stdin();
    let mut buffer: String = String::new();

    loop {
        print!("> ");
        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                run(buffer)
            }
            Err(error) => {
                println!("Error reading from repl...");
                process::exit(1);
            }
        }
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{token}");
    }
}

fn error(line: i32, message: String) {
    report(line, "", &message);
}

fn report(line: i32, is_where: &str, message: &str){
    eprintln!("[line {line}] Error {is_where}: {message}");
}