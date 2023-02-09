use once_cell::sync::Lazy;

use crate::token::{Token, TokenType};

static DEBUG_TOKENS: Lazy<bool> =
    Lazy::new(|| std::env::var("DEBUG_TOKENS").is_ok());

pub struct Lexer<'l> {
    source: &'l str,
    offset: usize,
    line_no: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            source,
            offset: 0,
            line_no: 1,
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
            None => return None,
            Some(char) => match char {
                "(" => {
                    let token = Token::new(
                        TokenType::LeftParen,
                        char.to_owned(),
                        self.line_no,
                    );

                    self.advance();

                    token
                }
                ")" => {
                    let token = Token::new(
                        TokenType::RightParen,
                        char.to_owned(),
                        self.line_no,
                    );

                    self.advance();

                    token
                }
                "\"" => self.string(),
                _ if self.is_number() => self.number(),
                _ => self.symbol(),
            },
        };

        if *DEBUG_TOKENS {
            println!("{token:#?}");
        }

        Some(token)
    }

    fn current_character(&self) -> Option<&'l str> {
        self.source.get(self.offset..=self.offset)
    }

    fn next_character(&self) -> Option<&'l str> {
        self.source.get(self.offset + 1..=self.offset + 1)
    }

    fn at_end(&self) -> bool {
        self.current_character() == Some("")
    }

    fn is_whitespace(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.chars().all(char::is_whitespace)
        } else {
            false
        }
    }

    fn is_number(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.chars().all(char::is_numeric)
        } else {
            false
        }
    }

    fn is_next_number(&self) -> bool {
        if let Some(next) = self.next_character() {
            next.chars().all(char::is_numeric)
        } else {
            false
        }
    }

    fn is_character(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.chars().all(|c| c.is_alphanumeric() || c == '_')
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        if self.current_character() == Some("#") {
            while self.current_character() != Some("\n") {
                self.advance();
            }

            self.advance();
        }
    }

    fn advance(&mut self) {
        self.offset += 1;

        if self.current_character() == Some("\n") {
            self.line_no += 1;
        }
    }

    fn string(&mut self) -> Token {
        // Advance past opening "
        self.advance();

        let mut string = String::new();

        while self.current_character() != Some("\"") {
            if self.at_end() {
                panic!("Unterminated string!");
            }

            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        self.advance();

        Token::new(TokenType::String, string, self.line_no)
    }

    fn number(&mut self) -> Token {
        let mut string = String::new();

        while self.is_number() {
            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        if self.current_character() == Some(".") {
            if !self.is_next_number() {
                panic!("Found decimal point not followed by decimals.")
            }

            string.push('.');
            self.advance();

            while self.is_number() {
                string.push_str(self.current_character().unwrap());
                self.advance();
            }
        }

        Token::new(TokenType::Number, string, self.line_no)
    }

    fn symbol(&mut self) -> Token {
        let mut string = String::new();

        while self.is_character() {
            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        Token::new(TokenType::Symbol, string, self.line_no)
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::{LeftParen, Number, RightParen, String, Symbol};

    fn lexer_test(source: &str, expected: &[Token]) {
        let lexer = Lexer::new(source);

        let output = lexer.collect::<Vec<_>>();

        assert_eq!(output, expected,);
    }

    fn token(ttype: TokenType, lexeme: &str) -> Token {
        Token::new(ttype, lexeme.to_owned(), 1)
    }

    fn number(number: f64) -> Token {
        token(Number, &number.to_string())
    }

    fn string(string: &str) -> Token {
        token(String, string)
    }

    fn symbol(string: &str) -> Token {
        token(Symbol, string)
    }

    #[test]
    fn test_empty_string() {
        lexer_test("", &[]);
    }

    #[test]
    fn test_parentheses() {
        lexer_test("()", &[token(LeftParen, "("), token(RightParen, ")")]);

        lexer_test(
            "())(",
            &[
                token(LeftParen, "("),
                token(RightParen, ")"),
                token(RightParen, ")"),
                token(LeftParen, "("),
            ],
        );
    }

    #[test]
    fn test_numbers() {
        lexer_test("1", &[number(1.)]);

        lexer_test("12.34", &[number(12.34)]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number() {
        lexer_test("12.", &[number(0.)]);
    }

    #[test]
    fn test_string() {
        lexer_test("\"\"", &[string("")]);

        lexer_test("\"This is a test.\"", &[string("This is a test.")]);
    }

    #[test]
    #[should_panic]
    fn test_unterminated_string() {
        lexer_test("\"Unterminated string", &[string("")]);
    }

    #[test]
    fn test_symbol() {
        lexer_test("symbol", &[symbol("symbol")]);
        lexer_test("[];',.", &[symbol("[];',.")]);
    }
}
