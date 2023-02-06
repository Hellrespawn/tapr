use crate::token::{Token, TokenType};

pub struct Lexer<'l> {
    source: &'l str,
    // symbol_table: &'l mut Vec<String>,
    offset: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(
        source: &'l str,
        // symbol_table: &'l mut Vec<String>
    ) -> Self {
        Self {
            source,
            // symbol_table,
            offset: 0,
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        if self.at_end() {
            return None;
        }

        self.skip_whitespace();
        self.skip_comments();

        let char = self.current_character();

        let token = match char {
            "(" => Token::new(TokenType::LeftParen, char.to_owned()),
            ")" => Token::new(TokenType::RightParen, char.to_owned()),
            "\"" => self.string(),
            _ if self.is_number() => self.number(),
            _ => self.symbol(),
        };

        self.advance();

        Some(token)
    }

    fn current_character(&self) -> &'l str {
        self.source.get(self.offset..self.offset + 1).unwrap()
    }

    fn at_end(&self) -> bool {
        self.offset > self.source.len()
    }

    fn is_whitespace(&self) -> bool {
        let current = self.current_character();
        current.chars().all(|c| c.is_whitespace())
    }

    fn is_number(&self) -> bool {
        let current = self.current_character();
        current.chars().all(|c| c.is_numeric())
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        if self.current_character() == "#" {
            while self.current_character() != "\n" {
                self.advance();
            }

            self.advance();
        }
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn string(&mut self) -> Token {
        todo!()
    }

    fn number(&mut self) -> Token {
        todo!()
    }

    fn symbol(&mut self) -> Token {
        todo!()
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}
