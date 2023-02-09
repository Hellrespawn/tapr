use rustyline::error::ReadlineError;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

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

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    loop {
        let readline = rl.readline("> ").map(|s| s.trim().to_owned());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    println!("Exiting REPL...");
                    break;
                }

                rl.add_history_entry(&line);

                run_code(&line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting REPL...");
                break;
            }
            Err(err) => {
                eprintln!("Error occured: {err}");
                break;
            }
        }
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

    // let tokens = lexer.collect::<Vec<_>>();

    // println!("{tokens:#?}");

    let mut parser = Parser::new(lexer);

    let program = parser.parse();

    let interpreter = Interpreter {};

    let output = interpreter.interpret(&program);

    println!("{output}");
}
