use crate::location::Location;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Apostrophe,
    Def,
    Defun,
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
    pub location: Location,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Token::{:?}", self.location, self.ttype)?;

        write!(f, "('{}')", self.lexeme)
    }
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, location: Location) -> Self {
        Self {
            ttype,
            lexeme,
            location,
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
