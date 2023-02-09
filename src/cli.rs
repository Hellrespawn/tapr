use std::io::Write;

use crate::lexer::Lexer;

pub fn main() {
    let mut args = std::env::args();

    match args.len() {
        1 => repl(),
        2 => run_file(&args.nth(1).expect("Args should be manually checked.")),
        _ => eprintln!("Usage: {} [FILENAME]", env!("CARGO_PKG_NAME")),
    }
}

fn repl() {
    println!("Welcome to Korisp.");

    let mut buffer = String::new();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("> ");
        stdout.flush().expect("Unable to flush stdout");

        stdin
            .read_line(&mut buffer)
            .expect("Unable to read from stdin");

        if buffer.trim().is_empty() {
            println!("Exiting REPL...");
            break;
        }

        run_code(&buffer);

        buffer.clear();
    }
}

fn run_file(filename: &str) {
    let source = std::fs::read_to_string(filename);

    match source {
        Ok(source) => {
            run_code(&source);
        }
        Err(error) => {
            eprintln!("Unable to open {filename}: {error}");
        }
    }
}

fn run_code(source: &str) {
    let lexer = Lexer::new(source);

    let tokens = lexer.collect::<Vec<_>>();

    println!("{tokens:#?}");
}
