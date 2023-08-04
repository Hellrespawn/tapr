use crate::location::Location;
use crate::node::NodeSource;
use crate::parser::Rule;
use crate::{Node, ParameterType};
use thiserror::Error;

#[derive(Debug)]
pub struct Error {
    pub source: NodeSource,
    pub kind: ErrorKind,
}
impl Error {
    pub fn new(source: NodeSource, kind: ErrorKind) -> Self {
        Self { source, kind }
    }

    pub fn has_source(&self) -> bool {
        !matches!(self.source, NodeSource::Unknown)
    }

    pub fn location(&self) -> Option<Location> {
        self.source.location()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.location() {
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
            source: NodeSource::Unknown,
            kind: error.into(),
        }
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self {
            source: NodeSource::Unknown,
            kind: Box::new(error).into(),
        }
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(error: rustyline::error::ReadlineError) -> Self {
        Self {
            source: NodeSource::Unknown,
            kind: error.into(),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            source: NodeSource::Unknown,
            kind,
        }
    }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("{0}")]
    Message(String),

    // CLI
    #[error("Usage: {} [FILENAME]", env!("CARGO_PKG_NAME"))]
    UsageError,

    #[error(transparent)]
    ParserError {
        #[from]
        source: Box<pest::error::Error<Rule>>,
    },

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

    #[error("Undefined symbol '{0}'")]
    SymbolNotDefined(String),

    #[error("Already defined symbol '{0}'")]
    SymbolDefined(String),

    #[error("Invalid argument '{actual}', expected '{expected:?}'")]
    InvalidValueArgument {
        expected: Vec<ParameterType>,
        actual: Node,
    },

    #[error("Invalid argument '{actual}', expected '{expected:?}'")]
    InvalidNodeArgument {
        expected: Vec<ParameterType>,
        actual: Node,
    },

    // Functions
    #[error("Called `tail` on empty list.")]
    TailOnEmptyList,

    #[error("Unable to {op} {lhs} and {rhs}")]
    InvalidBinOp {
        op: &'static str,
        lhs: Node,
        rhs: Node,
    },

    #[error("Unable to parse {0:?} as number")]
    ParseNumberError(String),

    #[error("Unable to parse {0} as integer.")]
    InvalidInteger(f64),

    #[error("Unable to call value:\n{0}")]
    NotCallable(Node),
}
