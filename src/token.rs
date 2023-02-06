#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,

    String,
    Number,
    Symbol,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String) -> Self {
        Self { ttype, lexeme }
    }
}
