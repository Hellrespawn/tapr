#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    String,
    Number,
    Symbol,

    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line_no: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line_no: usize) -> Self {
        Self {
            ttype,
            lexeme,
            line_no,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
}
