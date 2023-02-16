use crate::visitor::interpreter::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Cli
    #[error("Usage: {} [FILENAME]", env!("CARGO_PKG_NAME"))]
    UsageError,
    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    ReadLine {
        #[from]
        source: rustyline::error::ReadlineError,
    },

    // Lexer
    #[error("Unknown character '{character}' at {line_no}:{char_no}.")]
    UnknownCharacter {
        character: String,
        line_no: usize,
        char_no: usize,
    },
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Found decimal point not followed by decimals.")]
    DecimalPointNotFollowedByDigits,

    // Parser
    #[error("{0}")]
    ConsumeError(String),

    #[error("{0}")]
    Parser(String),

    // Interpreter
    #[error("Undefined symbol '{0}'")]
    UndefinedSymbol(String),
    #[error("Invalid operands '{values:?}', expected '{expected}'")]
    InvalidArguments {
        expected: &'static str,
        values: Vec<Value>,
    },
    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfArgs { expected: String, actual: usize },
}
