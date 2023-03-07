use self::ast::*;
use crate::error::{Error, ErrorKind};
use crate::graph::GraphVisitor;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::Result;
use once_cell::sync::Lazy;

pub mod ast;

static DEBUG_AST: Lazy<bool> = Lazy::new(|| std::env::var("DEBUG_AST").is_ok());

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    parser_no: usize,
    previous_token: Option<Token>,
    current_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            parser_no: 0,
            previous_token: None,
            current_token: None,
        }
    }

    pub fn with_number(lexer: Lexer<'p>, parser_no: usize) -> Self {
        Self {
            lexer,
            parser_no,
            previous_token: None,
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.advance()?;

        let expression = self.expression()?;

        if *DEBUG_AST {
            let filename = format!(
                "{}.{}.ast.dot",
                env!("CARGO_PKG_NAME"),
                self.parser_no
            );

            GraphVisitor::create_ast_graph(&expression, &filename);
        }

        Ok(expression)
    }

    fn advance(&mut self) -> Result<()> {
        self.previous_token = self.current_token.take();

        self.current_token = self.lexer.scan_token()?;

        Ok(())
    }

    fn error(line_no: usize, col_no: usize, kind: ErrorKind) -> Error {
        Error::new(line_no, col_no, kind)
    }

    fn error_at_current(&self, kind: ErrorKind) -> Error {
        let (line_no, col_no) = self.current_location();

        Self::error(line_no, col_no, kind)
    }

    fn error_at_previous(&self, kind: ErrorKind) -> Error {
        let (line_no, col_no) = self.previous_location();

        Self::error(line_no, col_no, kind)
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

    fn current_location(&self) -> (usize, usize) {
        if let Some(token) = self.current_token.as_ref() {
            (token.line_no, token.col_no)
        } else {
            (0, 0)
        }
    }

    fn previous_location(&self) -> (usize, usize) {
        if let Some(token) = self.previous_token.as_ref() {
            (token.line_no, token.col_no)
        } else {
            (0, 0)
        }
    }

    fn gather_expressions_until_paren(&mut self) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        let (line_no, col_no) = self.previous_location();

        while !self.matches(TokenType::RightParen) {
            if self.current_token.is_none() {
                return Err(Self::error(
                    line_no,
                    col_no,
                    ErrorKind::UnmatchedParenthesis,
                ));
            }

            expressions.push(self.expression()?);
        }

        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Expression> {
        todo!()
        // if let Some(TokenType::LeftParen) = self.current_type() {
        //     self.advance()?;

        //     let expression = match self.current_type().unwrap_or(TokenType::EOF)
        //     {
        //         TokenType::If => self.if_expression()?.into(),
        //         TokenType::While => self.while_expression()?.into(),
        //         TokenType::Set => self.set_expression()?.into(),
        //         TokenType::Def => self.function_definition()?.into(),
        //         TokenType::Symbol => self.function_call()?.into(),
        //         _ => self.list()?.into(),
        //     };

        //     Ok(expression)
        // } else {
        //     self.data_type()
        // }
    }

    fn define(&mut self) -> Result<Define> {
        todo!()
    }

    fn if_expression(&mut self) -> Result<If> {
        self.consume(TokenType::If, "First 'if'")?;

        let condition = Box::new(self.expression()?);

        let then_branch = Box::new(self.expression()?);

        let else_branch = if self.matches(TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.expression()?))
        };

        self.consume(TokenType::RightParen, "If Expression must end with ')'")?;

        Ok(If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_expression(&mut self) -> Result<While> {
        self.consume(
            TokenType::While,
            "While Expression must start with 'while'",
        )?;

        let condition = Box::new(self.expression()?);

        let expression = Box::new(self.expression()?);

        self.consume(
            TokenType::RightParen,
            "While Expression must end with ')'",
        )?;

        Ok(While {
            condition,
            expression,
        })
    }

    fn lambda(&mut self) -> Result<Lambda> {
        todo!()
        // self.consume(
        //     TokenType::Def,
        //     "Function definition must start with 'def'",
        // )?;

        // let name = self.consume_symbol_token(
        //     "Function definition must have symbol for name.",
        // )?;

        // let parameters = self.parameters()?;

        // let expression = Box::new(self.expression()?);

        // self.consume(
        //     TokenType::RightParen,
        //     "Function definition must end with ')'",
        // )?;

        // let function_definition = FunctionDefinition {
        //     name,
        //     parameters,
        //     expression,
        // };

        // Ok(function_definition)
    }

    fn call(&mut self) -> Result<Call> {
        todo!()
        // let name = self
        //     .consume_symbol_token("Function call must have symbol for name.")?;

        // let arguments = self.gather_expressions_until_paren()?;

        // self.advance()?;

        // Ok(FunctionCall { name, arguments })
    }

    fn datum(&mut self) -> Result<Datum> {
        todo!()
        // let (line_no, col_no) = self.previous_location();

        // let mut expressions: Vec<Node> = Vec::new();

        // while !self.matches(TokenType::RightParen) {
        //     if self.current_token.is_none() {
        //         return Err(Self::error(
        //             line_no,
        //             col_no,
        //             ErrorKind::UnmatchedParenthesis,
        //         ));
        //     }

        //     expressions.push(self.expression()?);
        // }

        // self.advance()?;

        // Ok(List { expressions })
    }
}
