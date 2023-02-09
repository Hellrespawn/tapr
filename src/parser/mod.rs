use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

use self::ast::{Atom, Expression, List, Node, Program};

pub mod ast;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    current_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
        };

        parser.advance();

        parser
    }

    pub fn parse(&mut self) -> Program {
        self.program()
    }

    fn error(message: &str) -> ! {
        panic!("{message}")
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.scan_token();
    }

    fn consume(&mut self, expected_type: TokenType) {
        if self.matches(expected_type) {
            self.advance();
        } else {
            Self::error("Expected type did not match actual type.")
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

    fn program(&mut self) -> Program {
        let mut expressions = Vec::new();

        while self.matches(TokenType::LeftParen) {
            expressions.push(self.expression());
        }

        if self.current_token.is_some() {
            Self::error("Program may only contain lists.");
        }

        Program { expressions }
    }

    fn expression(&mut self) -> Expression {
        self.consume(TokenType::LeftParen);

        if !self.matches(TokenType::Symbol) {
            Self::error("Expression must start with symbol.")
        }

        let symbol = self.current_token.as_ref().unwrap().lexeme().to_owned();

        self.advance();

        let mut arguments: Vec<Box<dyn Node>> = Vec::new();

        while self.current_token.is_some()
            && !self.matches(TokenType::RightParen)
        {
            if self.matches(TokenType::LeftParen) {
                arguments.push(Box::new(self.list()));
            } else {
                arguments.push(Box::new(self.atom()));
            }
        }

        self.consume(TokenType::RightParen);

        Expression { symbol, arguments }
    }

    fn list(&mut self) -> List {
        self.consume(TokenType::LeftParen);

        let mut elements: Vec<Box<dyn Node>> = Vec::new();

        while self.current_token.is_some()
            && !self.matches(TokenType::RightParen)
        {
            if self.matches(TokenType::LeftParen) {
                elements.push(Box::new(self.list()));
            } else {
                elements.push(Box::new(self.atom()));
            }
        }

        self.consume(TokenType::RightParen);

        List { elements }
    }

    fn atom(&mut self) -> Atom {
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

        self.advance();

        atom
    }
}
