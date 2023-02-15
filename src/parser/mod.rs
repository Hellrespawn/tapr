use self::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::visitor::graph::DotVisitor;
use crate::{Error, Result};
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

    pub fn parse(&mut self) -> Result<Node> {
        self.advance()?;

        let program = self.program()?;

        let node: Node = program.into();

        if *DEBUG_AST {
            let dot = DotVisitor::create_ast_dot(&node);

            std::fs::write(format!("{}.ast.dot", env!("CARGO_PKG_NAME")), dot)
                .expect("Unable to write dot.");
        }

        Ok(node)
    }

    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.scan_token()?;

        Ok(())
    }

    fn consume(
        &mut self,
        expected_type: TokenType,
        message: &str,
    ) -> Result<()> {
        if self.matches(expected_type) {
            self.advance()
        } else {
            Err(Error::ConsumeError(message.to_owned()))
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

    fn program(&mut self) -> Result<Program> {
        let mut expressions: Vec<Node> = Vec::new();

        while self.current_token.is_some() {
            expressions.push(self.expression()?);
        }

        Ok(Program { expressions })
    }

    fn expression(&mut self) -> Result<Node> {
        match self.current_type() {
            Some(TokenType::LeftParen) => {
                self.advance()?;

                let expression = if self.matches(TokenType::If) {
                    self.advance()?;
                    self.if_expression()?.into()
                } else if self.matches(TokenType::Var) {
                    self.advance()?;
                    self.var_expression()?.into()
                } else if self.matches(TokenType::Symbol) {
                    self.function_call()?.into()
                } else {
                    self.list()?.into()
                };

                self.consume(
                    TokenType::RightParen,
                    "Expected ')' after expression.",
                )?;

                Ok(expression)
            }
            _ => Ok(self.atom()?.into()),
        }
    }

    fn if_expression(&mut self) -> Result<IfExpression> {
        let condition = Box::new(self.expression()?);

        let then_branch = Box::new(self.expression()?);

        let else_branch = if self.matches(TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.expression()?))
        };

        Ok(IfExpression {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn var_expression(&mut self) -> Result<VarExpression> {
        let Atom::Symbol(name) = self.atom()? else {
            return Err(Error::Parser(
                "Set expression must be followed by a symbol.".to_owned(),
            ));
        };

        let expression = Box::new(self.expression()?);

        Ok(VarExpression { name, expression })
    }

    fn function_call(&mut self) -> Result<FunctionCall> {
        let Atom::Symbol(symbol) = self.atom()? else {
            return Err(Error::Parser(
                "Function call must be followed by a symbol.".to_owned(),
            ));
        };

        let mut expressions: Vec<Node> = Vec::new();

        while self.current_token.is_some()
            && !self.matches(TokenType::RightParen)
        {
            expressions.push(self.expression()?);
        }

        Ok(FunctionCall {
            name: symbol,
            arguments: expressions,
        })
    }

    fn list(&mut self) -> Result<List> {
        // Leading paren consumed by expression

        let mut elements: Vec<Node> = Vec::new();

        while self.current_token.is_some()
            && !self.matches(TokenType::RightParen)
        {
            elements.push(self.expression()?);
        }

        // Trailing paren consumed by expression

        Ok(List { elements })
    }

    fn atom(&mut self) -> Result<Atom> {
        let atom = match self.current_token.as_ref() {
            None => unreachable!(),
            Some(token) => match token.ttype {
                TokenType::True => Atom::Boolean(true),
                TokenType::False => Atom::Boolean(false),
                TokenType::Number => {
                    // Checked by lexer
                    Atom::Number(token.lexeme().parse::<f64>().unwrap())
                }
                TokenType::String => Atom::String(token.lexeme().to_owned()),
                TokenType::Symbol => Atom::Symbol(token.lexeme().to_owned()),
                TokenType::Nil => Atom::Nil,
                ttype => unreachable!("Invalid TokenType for Atom '{ttype:?}'"),
            },
        };

        self.advance()?;

        Ok(atom)
    }
}
