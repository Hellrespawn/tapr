use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::error::ErrorKind;
use crate::interpreter::{Interpreter, Value};
use crate::Result;

// TODO More sophisticated command line handling.
// TODO Save repl-history in a config dir
// TODO Allow resetting of REPL history

const HISTFILE: &str = "history.txt";

pub fn main() {
    let mut args = std::env::args();

    let result = match args.len() {
        1 => repl(),
        2 => run_file(&args.nth(1).expect("args to have 2 elements.")),
        _ => Err(ErrorKind::UsageError.into()),
    };

    if let Err(error) = result {
        eprintln!("{error}");
    }
}

fn repl() -> Result<()> {
    println!("Welcome to Tapr {}.", env!("CARGO_PKG_VERSION"));

    let mut rl = Editor::<()>::new().unwrap();

    let _result = rl.load_history(HISTFILE);
    let starting_index = std::fs::read_to_string(HISTFILE)
        .map(|s| s.trim().lines().count())
        .unwrap_or(1);

    let mut intp = Interpreter::default();

    for line_no in starting_index.. {
        if eval_line(&mut rl, line_no, &mut intp)? {
            break;
        }
    }

    let _result = rl.save_history(HISTFILE);
    println!("Exiting REPL...");

    Ok(())
}

fn eval_line(
    rl: &mut Editor<()>,
    line_no: usize,
    intp: &mut Interpreter,
) -> Result<bool> {
    let readline =
        rl.readline(&format!("[{line_no}]> ")).map(|s| s.trim().to_owned());

    match readline {
        Ok(line) => {
            if line.is_empty() {
                return Ok(true);
            }

            rl.add_history_entry(&line);

            match run_code(&line, intp, &format!("repl_{line_no}")) {
                Ok(value) => println!("{}", value.repl_repr()),
                Err(error) => eprintln!("{error}"),
            }

            Ok(false)
        },
        Err(ReadlineError::Interrupted) => Ok(true),
        Err(err) => Err(err.into()),
    }
}

fn run_file(filename: &str) -> Result<()> {
    let source = std::fs::read_to_string(filename)?;
    let mut intp = Interpreter::default();

    if let Err(error) = run_code(&source, &mut intp, filename) {
        eprintln!("{error}");
    }

    Ok(())
}

fn run_code(source: &str, intp: &mut Interpreter, name: &str) -> Result<Value> {
    intp.interpret(source, name)
}
