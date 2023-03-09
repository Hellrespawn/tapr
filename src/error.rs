use crate::interpreter::parameters::ParameterType;
use crate::interpreter::Value;
use crate::token::TokenType;
use thiserror::Error;

#[derive(Debug)]
pub struct Error {
    pub line_no: Option<usize>,
    pub col_no: Option<usize>,
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

    pub fn has_location(&self) -> bool {
        self.line_no.is_some() && self.col_no.is_some()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(line_no), Some(char_no)) = (self.line_no, self.col_no) {
            write!(f, "[{line_no}:{char_no}] {}", self.kind)
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
    #[error("Only the last parameter of a function may be variadic.")]
    NonLastParameterIsVariadic,

    #[error("Invalid argument '{actual}', expected '{expected:?}'")]
    InvalidArgument {
        expected: Vec<ParameterType>,
        actual: Value,
    },

    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfFixedArgs { expected: usize, actual: usize },

    #[error("Expect at least{expected} args, got {actual}.")]
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
}
