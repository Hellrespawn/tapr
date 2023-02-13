use crate::token::TokenType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    #[error("Unknown character '{0}'.")]
    UnknownCharacter(String),
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Found decimal point not followed by decimals.")]
    DecimalPointNotFollowedByDigits,
    #[error("Expected {expected:?}, found {found:?}")]
    ConsumeError {
        expected: TokenType,
        found: TokenType,
    },
}
