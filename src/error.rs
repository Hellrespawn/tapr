use crate::token::TokenType;
use crate::visitor::interpreter::Value;
use thiserror::Error;

// TODO add function error, handle in call_function, add line info.

#[derive(Debug)]
pub struct Error {
    line_no: Option<usize>,
    col_no: Option<usize>,
    pub kind: ErrorKind,
}
impl Error {
    pub fn new(line_no: usize, col_no: usize, kind: ErrorKind) -> Self {
        Self {
            line_no: Some(line_no),
            col_no: Some(col_no),
            kind,
        }
    }

    pub fn without_location(kind: ErrorKind) -> Self {
        Self {
            line_no: None,
            col_no: None,
            kind,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(line_no), Some(char_no)) = (self.line_no, self.col_no) {
            write!(f, "[{}:{}] {}", line_no, char_no, self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::without_location(error.into())
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(error: rustyline::error::ReadlineError) -> Self {
        Self::without_location(error.into())
    }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
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
    #[error("Unknown character '{0}'")]
    UnknownCharacter(char),
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Found decimal point not followed by decimals.")]
    DecimalPointNotFollowedByDigits,

    // Parser
    #[error("{0}")]
    ConsumeError(String),

    #[error("{0}")]
    ParserError(String),

    #[error("There was no input.")]
    EmptyInput,

    #[error("Unmatched parenthesis.")]
    UnmatchedParenthesis,

    #[error("Unable to parse Token as atom: '{0:?}'")]
    InvalidTypeForAtom(TokenType),

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
