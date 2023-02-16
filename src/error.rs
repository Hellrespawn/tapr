use crate::token::TokenType;
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
    #[error("{line_no}:{char_no}: Unknown character '{character}'")]
    UnknownCharacter {
        character: String,
        line_no: usize,
        char_no: usize,
    },
    #[error("{line_no}:{char_no}: Unterminated string.")]
    UnterminatedString { line_no: usize, char_no: usize },
    #[error(
        "{line_no}:{char_no}: Found decimal point not followed by decimals."
    )]
    DecimalPointNotFollowedByDigits { line_no: usize, char_no: usize },

    // Parser
    #[error("{line_no}:{char_no}: {message}")]
    ConsumeError {
        message: String,
        line_no: usize,
        char_no: usize,
    },

    #[error("{line_no}:{char_no}: {message}")]
    Parser {
        message: String,
        line_no: usize,
        char_no: usize,
    },

    #[error("{line_no}:{char_no}: Unable to parse as atom: '{ttype:?}'")]
    InvalidTypeForAtom {
        ttype: TokenType,
        line_no: usize,
        char_no: usize,
    },

    // Interpreter
    #[error("{line_no}:{char_no}: Undefined symbol '{symbol}'")]
    UndefinedSymbol {
        symbol: String,
        line_no: usize,
        char_no: usize,
    },
    #[error("Invalid operands '{values:?}', expected '{expected}'")]
    InvalidArguments {
        expected: &'static str,
        values: Vec<Value>,
    },
    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfArgs { expected: String, actual: usize },
}
