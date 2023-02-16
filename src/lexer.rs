use crate::token::{Token, TokenType};
use crate::{Error, Result};
use once_cell::sync::Lazy;

static DEBUG_TOKENS: Lazy<bool> =
    Lazy::new(|| std::env::var("DEBUG_TOKENS").is_ok());

static CHARACTERS: &str = "_-+*/!%^&'<>=";

pub struct Lexer<'l> {
    source: &'l str,
    offset: usize,
    char_no: usize,
    line_no: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            source,
            offset: 0,
            char_no: 1,
            line_no: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();
        self.skip_comments();

        let char = self.current_character();

        let token = match char {
            None => return Ok(None),
            Some(char) => match char {
                "(" => {
                    let token = Token::new(
                        TokenType::LeftParen,
                        char.to_owned(),
                        self.line_no,
                        self.char_no,
                    );

                    self.advance();

                    token
                }
                ")" => {
                    let token = Token::new(
                        TokenType::RightParen,
                        char.to_owned(),
                        self.line_no,
                        self.char_no,
                    );

                    self.advance();

                    token
                }
                "'" => {
                    let token = Token::new(
                        TokenType::Quote,
                        char.to_owned(),
                        self.line_no,
                        self.char_no,
                    );

                    self.advance();

                    token
                }
                "\"" => self.string()?,
                _ if self.is_number() => self.number()?,
                _ if self.is_character() => self.keyword_or_symbol(),
                _ => {
                    return Err(Error::UnknownCharacter {
                        character: char.to_owned(),
                        line_no: self.line_no,
                        char_no: self.char_no,
                    })
                }
            },
        };

        if *DEBUG_TOKENS {
            println!("{token:#?}");
        }

        Ok(Some(token))
    }

    fn current_character(&self) -> Option<&'l str> {
        self.source.get(self.offset..=self.offset)
    }

    fn next_character(&self) -> Option<&'l str> {
        self.source.get(self.offset + 1..=self.offset + 1)
    }

    fn at_end(&self) -> bool {
        self.current_character().is_none()
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
            current
                .chars()
                .all(|c| c.is_alphanumeric() || CHARACTERS.contains(c))
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

            self.skip_whitespace();
        }
    }

    fn advance(&mut self) {
        self.offset += 1;

        if self.current_character() == Some("\n") {
            self.line_no += 1;
            self.char_no = 1;
        } else {
            self.char_no += 1;
        }
    }

    fn string(&mut self) -> Result<Token> {
        let mut string = String::new();

        // Preserve location at start of string.
        let (line_no, char_no) = (self.line_no, self.char_no);

        // Advance past opening "
        self.advance();

        while self.current_character() != Some("\"") {
            if self.at_end() {
                return Err(Error::UnterminatedString { line_no, char_no });
            }

            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        // Advance past closing "
        self.advance();

        let token = Token::new(TokenType::String, string, line_no, char_no);

        Ok(token)
    }

    fn number(&mut self) -> Result<Token> {
        let mut string = String::new();

        // Preserve location at start of number.
        let (line_no, char_no) = (self.line_no, self.char_no);

        while self.is_number() {
            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        if self.current_character() == Some(".") {
            if !self.is_next_number() {
                return Err(Error::DecimalPointNotFollowedByDigits {
                    line_no,
                    char_no,
                });
            }

            string.push('.');
            self.advance();

            while self.is_number() {
                string.push_str(self.current_character().unwrap());
                self.advance();
            }
        }

        let token = Token::new(TokenType::Number, string, line_no, char_no);

        Ok(token)
    }

    fn keyword_or_symbol(&mut self) -> Token {
        let mut string = String::new();

        // Preserve location at start of string.
        let (line_no, char_no) = (self.line_no, self.char_no);

        while self.is_character() {
            string.push_str(self.current_character().unwrap());
            self.advance();
        }

        let ttype = match string.as_str() {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            "if" => TokenType::If,
            "while" => TokenType::While,
            "var" => TokenType::Var,
            _ => TokenType::Symbol,
        };

        Token::new(ttype, string, line_no, char_no)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::{LeftParen, Number, RightParen, String, Symbol};

    fn lexer_test(source: &str, expected: &[Token]) -> Result<()> {
        let mut lexer = Lexer::new(source);

        let mut output = Vec::new();

        while let Some(token) = lexer.scan_token()? {
            output.push(token);
        }

        assert_eq!(output, expected,);

        Ok(())
    }

    fn token(
        ttype: TokenType,
        lexeme: &str,
        line_no: usize,
        char_no: usize,
    ) -> Token {
        Token::new(ttype, lexeme.to_owned(), line_no, char_no)
    }

    fn number(number: f64, line_no: usize, char_no: usize) -> Token {
        token(Number, &number.to_string(), line_no, char_no)
    }

    fn string(string: &str, line_no: usize, char_no: usize) -> Token {
        token(String, string, line_no, char_no)
    }

    fn symbol(string: &str, line_no: usize, char_no: usize) -> Token {
        token(Symbol, string, line_no, char_no)
    }

    #[test]
    fn test_empty_string() -> Result<()> {
        lexer_test("", &[])?;

        Ok(())
    }

    #[test]
    fn test_parentheses() -> Result<()> {
        lexer_test(
            "()",
            &[token(LeftParen, "(", 1, 1), token(RightParen, ")", 1, 2)],
        )?;

        lexer_test(
            "())(",
            &[
                token(LeftParen, "(", 1, 1),
                token(RightParen, ")", 1, 2),
                token(RightParen, ")", 1, 3),
                token(LeftParen, "(", 1, 4),
            ],
        )?;

        Ok(())
    }

    #[test]
    fn test_numbers() -> Result<()> {
        lexer_test("1", &[number(1., 1, 1)])?;

        lexer_test("12.34", &[number(12.34, 1, 1)])?;

        Ok(())
    }

    #[test]
    fn test_invalid_number() {
        let result = lexer_test("12.", &[number(0., 0, 0)]);

        assert!(matches!(
            result,
            Err(Error::DecimalPointNotFollowedByDigits { .. })
        ));
    }

    #[test]
    fn test_string() -> Result<()> {
        lexer_test("\"\"", &[string("", 1, 1)])?;

        lexer_test("\"This is a test.\"", &[string("This is a test.", 1, 1)])?;

        Ok(())
    }

    #[test]
    fn test_unterminated_string() {
        let result = lexer_test("\"Unterminated string", &[string("", 0, 0)]);

        assert!(matches!(result, Err(Error::UnterminatedString { .. })));
    }

    #[test]
    fn test_symbol() -> Result<()> {
        lexer_test("symbol", &[symbol("symbol", 1, 1)])?;
        lexer_test("+-_", &[symbol("+-_", 1, 1)])?;

        Ok(())
    }

    #[test]
    fn test_invalid_characters() {
        let result = lexer_test("[]", &[]);

        assert!(matches!(
            result,
            Err(Error::UnknownCharacter{ character: string, ..}) if string == "["
        ));
    }

    #[test]
    fn test_whitespace_before_left_paren() -> Result<()> {
        lexer_test(
            "  ()",
            &[token(LeftParen, "(", 1, 3), token(RightParen, ")", 1, 4)],
        )
    }
}
