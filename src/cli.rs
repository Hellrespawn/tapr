use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::Result;
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
    println!("Welcome to Korisp.");

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    loop {
        let readline = rl.readline("> ").map(|s| s.trim().to_owned());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    break;
                }

                rl.add_history_entry(&line);

                let result = run_code(&line);

                if let Err(error) = result {
                    eprintln!("{error}");
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
    run_code(&source)
}

fn run_code(source: &str) -> Result<()> {
    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);

    let program = parser.parse()?;

    println!("{program:#?}");

    Ok(())
}
