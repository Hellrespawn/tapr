use self::ast::{Atom, List, Node, Program};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::Result;
use once_cell::sync::Lazy;

pub mod ast;

static DEBUG_AST: Lazy<bool> = Lazy::new(|| std::env::var("DEBUG_AST").is_ok());

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    current_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        self.advance()?;

        let program = self.program()?;

        if *DEBUG_AST {
            println!("{program:#?}");
        }

        Ok(program)
    }

    fn error(message: &str) -> ! {
        panic!("{message}")
    }

    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.scan_token()?;

        Ok(())
    }

    fn consume(&mut self, expected_type: TokenType) -> Result<()> {
        if self.matches(expected_type) {
            self.advance()
        } else {
            Err(Error::ConsumeError {
                expected: expected_type,
                found: self
                    .current_token
                    .as_ref()
                    .map_or(TokenType::EOF, |t| t.ttype),
            })
        }
    }

    fn matches(&mut self, expected_type: TokenType) -> bool {
        if let Some(token) = self.current_token.as_ref() {
            if token.ttype == expected_type {
                return true;
            }
        }

        false
    }

    fn program(&mut self) -> Result<Program> {
        let mut lists = Vec::new();

        while self.matches(TokenType::LeftParen) {
            lists.push(self.list()?);
        }

        if self.current_token.is_some() {
            Self::error("Program may only contain lists.");
        }

        Ok(Program { lists })
    }

    fn list(&mut self) -> Result<List> {
        self.consume(TokenType::LeftParen)?;

        let mut elements: Vec<Node> = Vec::new();

        while self.current_token.is_some()
            && !self.matches(TokenType::RightParen)
        {
            if self.matches(TokenType::LeftParen) {
                elements.push(self.list()?.into());
            } else {
                elements.push(self.atom()?.into());
            }
        }

        self.consume(TokenType::RightParen)?;

        Ok(List { elements })
    }

    fn atom(&mut self) -> Result<Atom> {
        let atom = match self.current_token.as_ref() {
            None => unreachable!(),
            Some(token) => match token.ttype {
                TokenType::Number => {
                    // Checked by lexer
                    Atom::Number(token.lexeme().parse::<f64>().unwrap())
                }
                TokenType::String => Atom::String(token.lexeme().to_owned()),
                TokenType::Symbol => Atom::Symbol(token.lexeme().to_owned()),
                _ => Self::error("Invalid TokenType for Atom."),
            },
        };

        self.advance()?;

        Ok(atom)
    }
}
