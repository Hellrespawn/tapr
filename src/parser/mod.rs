use self::ast::*;
use crate::error::{Error, ErrorKind};
use crate::graph::GraphVisitor;
use crate::lexer::Lexer;
use crate::location::Location;
use crate::token::{Token, TokenType, TokenType as TT};
use crate::Result;
use once_cell::sync::Lazy;

pub mod ast;

static DEBUG_AST: Lazy<bool> = Lazy::new(|| std::env::var("DEBUG_AST").is_ok());

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    previous_token: Option<Token>,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            previous_token: None,
            current_token: None,
            next_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.advance()?;

        if self.current_type().unwrap_or(TokenType::EOF) == TokenType::EOF {
            return Err(ErrorKind::EmptyInput.into());
        }

        let expression = self.expression()?;

        if *DEBUG_AST {
            let filename = format!("{}.ast.dot", env!("CARGO_PKG_NAME"));

            GraphVisitor::create_ast_graph(&expression, &filename);
        }

        Ok(expression)
    }

    fn peek(&mut self) -> Result<Option<&Token>> {
        if self.next_token.is_none() {
            self.next_token = self.lexer.scan_token()?;
        }

        Ok(self.next_token.as_ref())
    }

    fn advance(&mut self) -> Result<()> {
        self.previous_token = self.current_token.take();

        if self.next_token.is_some() {
            self.current_token = self.next_token.take();
        } else {
            self.current_token = self.lexer.scan_token()?;
        }

        Ok(())
    }

    fn error(location: Location, kind: ErrorKind) -> Error {
        Error::new(location, kind)
    }

    fn error_at_previous(&self, kind: ErrorKind) -> Error {
        Self::error(self.previous_location(), kind)
    }

    fn consume(
        &mut self,
        expected_type: TokenType,
        message: &str,
    ) -> Result<()> {
        if self.matches(expected_type) {
            self.advance()
        } else {
            Err(self
                .error_at_previous(ErrorKind::ConsumeError(message.to_owned())))
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

    fn current_type(&self) -> Option<TokenType> {
        self.current_token.as_ref().map(|token| token.ttype)
    }

    fn previous_location(&self) -> Location {
        if let Some(token) = self.previous_token.as_ref() {
            token.location
        } else {
            Location::new(0, 0)
        }
    }

    fn gather_expressions_until_paren(&mut self) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        let location = self.previous_location();

        while !self.matches(TokenType::RightParen) {
            if self.current_token.is_none() {
                return Err(Self::error(
                    location,
                    ErrorKind::UnmatchedParenthesis,
                ));
            }

            expressions.push(self.expression()?);
        }

        self.advance()?;

        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Expression> {
        let current_type = self.current_type().unwrap_or(TokenType::EOF);

        let next_type = self.peek()?.map_or(TokenType::EOF, |t| t.ttype);

        let expr = match (current_type, next_type) {
            (TT::LeftParen, TT::Def) => Expression::Define(self.define()?),
            (TT::LeftParen, TT::Defun) => Expression::Define(self.defun()?),
            (TT::LeftParen, TT::If) => Expression::If(self.if_expression()?),
            (TT::LeftParen, TT::While) => {
                Expression::While(self.while_expression()?)
            }
            (TT::LeftParen, TT::Lambda) => Expression::Lambda(self.lambda()?),
            (TT::LeftParen, TT::Symbol) => Expression::Call(self.call()?),
            (TT::LeftParen, TT::Quote) => {
                Expression::QuotedDatum(self.quoted_datum(false)?)
            }
            (TT::Apostrophe, _) => {
                Expression::QuotedDatum(self.quoted_datum(true)?)
            }
            _ => Expression::Datum(self.datum()?),
        };

        Ok(expr)
    }

    fn define(&mut self) -> Result<Define> {
        self.consume(TokenType::LeftParen, "")?;
        self.consume(TokenType::Def, "")?;

        let name = self.symbol("Define should have a symbol.")?;

        let expression = Box::new(self.expression()?);

        self.consume(TokenType::RightParen, "Define should be closed by ')'")?;

        Ok(Define { name, expression })
    }

    fn defun(&mut self) -> Result<Define> {
        self.consume(TokenType::LeftParen, "")?;
        self.consume(TokenType::Defun, "")?;

        let name = self.symbol("Defun should have a symbol.")?;

        let parameters = self.parameters()?;

        let expression = Box::new(self.expression()?);

        let lambda = Lambda {
            parameters,
            expression,
        };

        self.consume(TokenType::RightParen, "Defun should be closed by ')'")?;

        Ok(Define {
            name,
            expression: Box::new(Expression::Lambda(lambda)),
        })
    }

    fn if_expression(&mut self) -> Result<If> {
        self.consume(TokenType::LeftParen, "")?;
        self.consume(TokenType::If, "")?;

        let condition = Box::new(self.expression()?);

        let then_branch = Box::new(self.expression()?);

        let else_branch = if self.matches(TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.expression()?))
        };

        self.consume(TokenType::RightParen, "If should be closed by ')'")?;

        Ok(If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_expression(&mut self) -> Result<While> {
        self.consume(TokenType::LeftParen, "")?;
        self.consume(TokenType::While, "")?;

        let condition = Box::new(self.expression()?);

        let expression = Box::new(self.expression()?);

        self.consume(TokenType::RightParen, "While should be closed by ')'")?;

        Ok(While {
            condition,
            expression,
        })
    }

    fn lambda(&mut self) -> Result<Lambda> {
        self.consume(TokenType::LeftParen, "")?;
        self.consume(TokenType::Lambda, "")?;

        let parameters = self.parameters()?;

        let expression = Box::new(self.expression()?);

        self.consume(TokenType::RightParen, "Lambda should be closed by ')'")?;

        Ok(Lambda {
            parameters,
            expression,
        })
    }

    fn parameters(&mut self) -> Result<Vec<Symbol>> {
        self.consume(TokenType::LeftParen, "Parameters should open with '('.")?;

        let mut parameters = Vec::new();

        while !self.matches(TokenType::RightParen) {
            parameters
                .push(self.symbol("Lambda parameter should be a symbol.")?);
        }

        self.advance()?;

        Ok(parameters)
    }

    fn call(&mut self) -> Result<Call> {
        self.consume(TokenType::LeftParen, "")?;

        let symbol = self.symbol("Call should start with a symbol.")?;

        let arguments = self.gather_expressions_until_paren()?;

        Ok(Call { symbol, arguments })
    }

    fn quoted_datum(&mut self, shorthand: bool) -> Result<Datum> {
        if shorthand {
            self.consume(TokenType::Apostrophe, "")?;
        } else {
            self.consume(TokenType::LeftParen, "")?;
            self.consume(TokenType::Quote, "")?;
        }

        let result = self.datum();

        if !shorthand {
            self.consume(
                TokenType::RightParen,
                "Quote should be closed by ')'.",
            )?;
        }

        result
    }

    fn datum(&mut self) -> Result<Datum> {
        if self.matches(TokenType::LeftParen) {
            Ok(Datum::List(self.list()?))
        } else {
            self.atom()
        }
    }

    fn list(&mut self) -> Result<List> {
        self.consume(TokenType::LeftParen, "")?;

        let expressions = self.gather_expressions_until_paren()?;

        let start_token =
            self.previous_token.as_ref().cloned().expect(
                "Previous token should be `Some` after successful consume",
            );

        Ok(List {
            start_token,
            expressions,
        })
    }

    fn atom(&mut self) -> Result<Datum> {
        let current_token = self
            .current_token
            .as_ref()
            .cloned()
            .expect("This should not be None");

        let datum = match current_token.ttype {
            TokenType::True | TokenType::False => {
                self.advance()?;
                Datum::Boolean(Boolean(current_token))
            }
            TokenType::Number => {
                self.advance()?;
                Datum::Number(Number(current_token))
            }
            TokenType::String => {
                self.advance()?;
                Datum::String(StringNode(current_token))
            }
            TokenType::Symbol => Datum::Symbol(self.symbol("")?),
            TokenType::Nil => {
                self.advance()?;
                Datum::Nil
            }
            _ => {
                return Err(Error::new(
                    current_token.location,
                    ErrorKind::ParserError(format!(
                        "Invalid atom '{current_token:?}'"
                    )),
                ))
            }
        };

        Ok(datum)
    }

    fn symbol(&mut self, message: &str) -> Result<Symbol> {
        self.consume(TokenType::Symbol, message)?;

        let token =
            self.previous_token.as_ref().cloned().expect(
                "Previous token should be set after successful consume",
            );

        Ok(Symbol(token))
    }
}
