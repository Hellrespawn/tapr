use crate::interpreter::{ParameterType, Value};
use crate::location::Location;
use crate::token::TokenType;
use thiserror::Error;

#[derive(Debug)]
pub struct Error {
    pub location: Option<Location>,
    pub kind: ErrorKind,
}
impl Error {
    pub fn new(location: Location, kind: ErrorKind) -> Self {
        Self {
            location: Some(location),
            kind,
        }
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.location {
            write!(f, "{location} {}", self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            location: None,
            kind: error.into(),
        }
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(error: rustyline::error::ReadlineError) -> Self {
        Self {
            location: None,
            kind: error.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    // CLI
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

    #[error("Value '{0}' is not callable.")]
    NotCallable(Value),

    // Parameters
    #[error("Only the last parameter of a function may be a rest parameter.")]
    NonLastParameterIsRest,

    #[error("Invalid argument '{actual}', expected '{expected:?}'")]
    InvalidArgument {
        expected: ParameterType,
        actual: Value,
    },

    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfFixedArgs { expected: usize, actual: usize },

    #[error("Expect at least {expected} args, got {actual}.")]
    WrongAmountOfMinArgs { expected: usize, actual: usize },

    // Functions
    #[error("Called `tail` on empty list.")]
    TailOnEmptyList,

    #[error("Unable to {op} {lhs} and {rhs}")]
    InvalidBinOp {
        op: &'static str,
        lhs: Value,
        rhs: Value,
    },

    #[error("Unable to parse {0:?} as number")]
    ParseNumberError(String),
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            location: None,
            kind,
        }
    }
}
