#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Def,
    False,
    If,
    Lambda,
    LeftParen,
    Quote,
    RightParen,
    True,
    While,

    // Terminal rules
    String,
    Number,
    Symbol,
    Nil,

    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line_no: usize,
    pub col_no: usize,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: String,
        line_no: usize,
        col_no: usize,
    ) -> Self {
        Self {
            ttype,
            lexeme,
            line_no,
            col_no,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.lexeme() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        self.lexeme().parse::<f64>().ok()
    }
}
