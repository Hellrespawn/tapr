use crate::error::{Error, ErrorKind};
use crate::interpreter::{Interpreter, Value};
use crate::Result;
use rustyline::error::ReadlineError;

pub fn main() {
    let mut args = std::env::args();

    let result = match args.len() {
        1 => repl(),
        2 => run_file(&args.nth(1).expect("args to have 2 elements.")),
        _ => Err(Error::without_location(ErrorKind::UsageError)),
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }
}

fn repl() -> Result<()> {
    println!("Welcome to Korisp {}.", env!("CARGO_PKG_VERSION"));

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    let mut intp = Interpreter::repl();

    loop {
        let readline = rl.readline("> ").map(|s| s.trim().to_owned());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    break;
                }

                rl.add_history_entry(&line);

                let _result = run_code(&line, &mut intp);
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
    let mut intp = Interpreter::default();

    if let Err(error) = run_code(&source, &mut intp) {
        eprintln!("{error}");
    }

    Ok(())
}

fn run_code(source: &str, intp: &mut Interpreter) -> Result<Value> {
    intp.interpret(source)
}
