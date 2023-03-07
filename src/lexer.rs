use crate::error::{Error, ErrorKind};
use crate::token::{Token, TokenType};
use crate::Result;
use once_cell::sync::Lazy;

static DEBUG_TOKENS: Lazy<bool> =
    Lazy::new(|| std::env::var("DEBUG_TOKENS").is_ok());

static CHARACTERS: &str = "_-+*/!%^&'<>=";

/// `_Lex_ical analyz_er_` takes an input string and converts it to tokens
/// based on the terminal rules and constant characters of the grammar.
pub struct Lexer<'l> {
    source: &'l str,
    offset: usize,
    line_no: usize,
    col_no: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            source,
            offset: 0,
            col_no: 1,
            line_no: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Option<Token>> {
        self.skip_ignored_characters();

        if let Some(character) = self.current_character() {
            let token = match character {
                '(' => self.single_character(TokenType::LeftParen),
                ')' => self.single_character(TokenType::RightParen),
                '\'' => self.single_character(TokenType::Quote),
                '\"' => self.string()?,
                _ if self.is_number() => self.number()?,
                _ if self.is_character() => self.keyword_or_symbol(),
                _ => {
                    return Err(self.error_at_current(
                        ErrorKind::UnknownCharacter(character),
                    ))
                }
            };

            if *DEBUG_TOKENS {
                println!("{token:#?}");
            }

            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    fn error(line_no: usize, col_no: usize, kind: ErrorKind) -> Error {
        Error::new(line_no, col_no, kind)
    }

    fn error_at_current(&self, kind: ErrorKind) -> Error {
        Self::error(self.line_no, self.col_no, kind)
    }

    fn get_character(&self, offset: usize) -> Option<char> {
        self.source.as_bytes().get(offset).map(|b| *b as char)
    }

    fn current_character(&self) -> Option<char> {
        self.get_character(self.offset)
    }

    fn next_character(&self) -> Option<char> {
        self.get_character(self.offset + 1)
    }

    fn at_end(&self) -> bool {
        self.current_character().is_none()
    }

    fn is_whitespace(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.is_whitespace()
        } else {
            false
        }
    }

    fn is_number(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.is_numeric()
        } else {
            false
        }
    }

    fn is_next_number(&self) -> bool {
        if let Some(next) = self.next_character() {
            next.is_numeric()
        } else {
            false
        }
    }

    fn is_character(&self) -> bool {
        if let Some(current) = self.current_character() {
            current.is_alphanumeric() || CHARACTERS.contains(current)
        } else {
            false
        }
    }

    fn skip_ignored_characters(&mut self) {
        self.skip_whitespace();
        self.skip_comments();
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        while self.current_character() == Some('#') {
            while self.current_character() != Some('\n') {
                self.advance();
            }

            self.skip_whitespace();
        }
    }

    fn advance(&mut self) {
        self.offset += 1;

        if self.current_character() == Some('\n') {
            self.line_no += 1;
            self.col_no = 1;
        } else {
            self.col_no += 1;
        }
    }

    fn single_character(&mut self, ttype: TokenType) -> Token {
        let token = Token::new(
            ttype,
            self.current_character().unwrap().to_string(),
            self.line_no,
            self.col_no,
        );

        self.advance();

        token
    }

    fn string(&mut self) -> Result<Token> {
        let mut string = String::new();

        // Preserve location at start of string.
        let (line_no, col_no) = (self.line_no, self.col_no);

        // Advance past opening "
        self.advance();

        while self.current_character() != Some('"') {
            if self.at_end() {
                return Err(Self::error(
                    line_no,
                    col_no,
                    ErrorKind::UnterminatedString,
                ));
            }

            string.push(self.current_character().unwrap());
            self.advance();
        }

        // Advance past closing "
        self.advance();

        let token = Token::new(TokenType::String, string, line_no, col_no);

        Ok(token)
    }

    fn number(&mut self) -> Result<Token> {
        let mut string = String::new();

        // Preserve location at start of number.
        let (line_no, col_no) = (self.line_no, self.col_no);

        while self.is_number() {
            string.push(self.current_character().unwrap());
            self.advance();
        }

        if self.current_character() == Some('.') {
            if !self.is_next_number() {
                return Err(Self::error(
                    line_no,
                    col_no,
                    ErrorKind::DecimalPointNotFollowedByDigits,
                ));
            }

            string.push('.');
            self.advance();

            while self.is_number() {
                string.push(self.current_character().unwrap());
                self.advance();
            }
        }

        let token = Token::new(TokenType::Number, string, line_no, col_no);

        Ok(token)
    }

    fn keyword_or_symbol(&mut self) -> Token {
        let mut string = String::new();

        // Preserve location at start of string
        let (line_no, col_no) = (self.line_no, self.col_no);

        while self.is_character() {
            string.push(self.current_character().unwrap());
            self.advance();
        }

        let ttype = match string.as_str() {
            "def" => TokenType::Def,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "lambda" => TokenType::Lambda,
            "true" => TokenType::True,
            "while" => TokenType::While,
            "nil" => TokenType::Nil,
            _ => TokenType::Symbol,
        };

        Token::new(ttype, string, line_no, col_no)
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
        col_no: usize,
    ) -> Token {
        Token::new(ttype, lexeme.to_owned(), line_no, col_no)
    }

    fn number(number: f64, line_no: usize, col_no: usize) -> Token {
        token(Number, &number.to_string(), line_no, col_no)
    }

    fn string(string: &str, line_no: usize, col_no: usize) -> Token {
        token(String, string, line_no, col_no)
    }

    fn symbol(string: &str, line_no: usize, col_no: usize) -> Token {
        token(Symbol, string, line_no, col_no)
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
        let error = lexer_test("12.", &[number(0., 0, 0)]).unwrap_err();

        assert!(matches!(
            error.kind,
            ErrorKind::DecimalPointNotFollowedByDigits { .. }
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
        let error = lexer_test("\"Unterminated string", &[string("", 0, 0)])
            .unwrap_err();

        assert!(matches!(error.kind, ErrorKind::UnterminatedString { .. }));
    }

    #[test]
    fn test_symbol() -> Result<()> {
        lexer_test("symbol", &[symbol("symbol", 1, 1)])?;
        lexer_test("+-_", &[symbol("+-_", 1, 1)])?;

        Ok(())
    }

    #[test]
    fn test_invalid_characters() {
        let error = lexer_test("[]", &[]).unwrap_err();

        assert!(matches!(
            error.kind,
            ErrorKind::UnknownCharacter(character) if character == '['
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
