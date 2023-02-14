use crate::interpreter::{Interpreter, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::{Error, Result};
use rustyline::error::ReadlineError;

pub fn main() {
    let mut args = std::env::args();

    let result = match args.len() {
        1 => repl(),
        2 => run_file(&args.nth(1).expect("Args should be manually checked.")),
        _ => Err(Error::UsageError),
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }
}

fn repl() -> Result<()> {
    println!("Welcome to Korisp {}.", env!("CARGO_PKG_VERSION"));

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    let mut intp = Interpreter::new();

    loop {
        let readline = rl.readline("> ").map(|s| s.trim().to_owned());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    break;
                }

                rl.add_history_entry(&line);

                let result = run_code(&line, &mut intp);

                match result {
                    Ok(value) => println!("{value}"),
                    Err(error) => eprintln!("{error}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    println!("Exiting REPL...");

    Ok(())
}

fn run_file(filename: &str) -> Result<()> {
    let source = std::fs::read_to_string(filename)?;
    let mut intp = Interpreter::new();
    run_code(&source, &mut intp)?;
    Ok(())
}

fn run_code(source: &str, intp: &mut Interpreter) -> Result<Value> {
    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);

    let program = parser.parse()?;

    intp.interpret(&program)
}
