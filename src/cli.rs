use crate::error::ErrorKind;
use crate::graph::GraphVisitor;
use crate::interpreter::{Interpreter, Value};
use crate::parser::DEBUG_AST;
// use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Node;
use crate::Result;
use rustyline::error::ReadlineError;

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
    println!("Welcome to Tasp {}.", env!("CARGO_PKG_VERSION"));

    let mut rl = rustyline::Editor::<()>::new().unwrap();

    let _result = rl.load_history(HISTFILE);

    let mut intp = Interpreter::default();

    loop {
        let readline = rl.readline("> ").map(|s| s.trim().to_owned());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    break;
                }

                rl.add_history_entry(&line);

                match run_code(&line, &mut intp, "repl") {
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

    let _result = rl.save_history(HISTFILE);
    println!("Exiting REPL...");

    Ok(())
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
    let node = Node::from_string(source)?;

    if *DEBUG_AST {
        GraphVisitor::create_ast_graph(&node, name);
    }

    node.accept(intp)
}
