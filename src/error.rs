use crate::token::TokenType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
