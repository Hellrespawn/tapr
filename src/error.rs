use crate::location::Location;
use crate::parser::Rule;
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

impl From<pest::error::Error<Rule>> for Error {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self {
            location: None,
            kind: Box::new(error).into(),
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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            location: None,
            kind,
        }
    }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
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
}
