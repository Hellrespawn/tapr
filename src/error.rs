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
    #[error("{line_no}:{col_no}: Unknown character '{character}'")]
    UnknownCharacter {
        character: char,
        line_no: usize,
        col_no: usize,
    },
    #[error("{line_no}:{col_no}: Unterminated string.")]
    UnterminatedString { line_no: usize, col_no: usize },
    #[error(
        "{line_no}:{col_no}: Found decimal point not followed by decimals."
    )]
    DecimalPointNotFollowedByDigits { line_no: usize, col_no: usize },

    // Parser
    #[error("{line_no}:{col_no}: {message}")]
    ConsumeError {
        message: String,
        line_no: usize,
        col_no: usize,
    },

    #[error("{line_no}:{col_no}: {message}")]
    Parser {
        message: String,
        line_no: usize,
        col_no: usize,
    },

    #[error("There was no input.")]
    EmptyInput,

    #[error("{line_no}:{col_no}: Unmatched parenthesis.")]
    UnmatchedParenthesis { line_no: usize, col_no: usize },

    #[error("{line_no}:{col_no}: Unable to parse Token as atom: '{ttype:?}'")]
    InvalidTypeForAtom {
        ttype: TokenType,
        line_no: usize,
        col_no: usize,
    },

    // Interpreter
    #[error("{line_no}:{col_no}: Undefined symbol '{symbol}'")]
    UndefinedSymbol {
        symbol: String,
        line_no: usize,
        col_no: usize,
    },
    #[error("Invalid operands '{values:?}', expected '{expected}'")]
    InvalidArguments {
        expected: &'static str,
        values: Vec<Value>,
    },
    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfArgs { expected: String, actual: usize },
}
