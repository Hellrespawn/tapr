#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    String,
    Number,
    Symbol,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line_no: usize,
}

impl Token {
    #[must_use]
    pub fn new(ttype: TokenType, lexeme: String, line_no: usize) -> Self {
        Self {
            ttype,
            lexeme,
            line_no,
        }
    }
}
