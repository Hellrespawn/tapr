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
    previous_token: Option<Token>,
    current_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            previous_token: None,
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
        self.previous_token = self.current_token.take();

        self.current_token = self.lexer.scan_token()?;

        Ok(())
    }

    fn consume(
        &mut self,
        expected_type: TokenType,
        message: &str,
    ) -> Result<()> {
        if self.matches(expected_type) {
            self.advance()?;
            Ok(())
        } else {
            let (line_no, char_no) = self.previous_location();

            Err(Error::ConsumeError {
                message: message.to_owned(),
                line_no,
                char_no,
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

    fn current_type(&self) -> Option<TokenType> {
        self.current_token.as_ref().map(|token| token.ttype)
    }

    fn previous_location(&self) -> (usize, usize) {
        if let Some(token) = self.previous_token.as_ref() {
            (token.line_no, token.char_no)
        } else {
            (0, 0)
        }
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
            let (line_no, char_no) = self.previous_location();

            return Err(
                Error::Parser{
                    message: "Var expression must be followed by a symbol.".to_owned(),
                    line_no,
                    char_no
                }
            );
        };

        let expression = Box::new(self.expression()?);

        Ok(VarExpression { name, expression })
    }

    fn function_call(&mut self) -> Result<FunctionCall> {
        let Atom::Symbol(symbol) = self.atom()? else {
            let (line_no, char_no) = self.previous_location();

            return Err(
                Error::Parser{
                    message: "Function call must be followed by a symbol.".to_owned(),
                    line_no,
                    char_no
                }
            );
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
            Some(token) => {
                match token.ttype {
                    TokenType::True | TokenType::False => {
                        Atom::Boolean(token.clone())
                    }
                    TokenType::Number => {
                        // Checked by lexer
                        Atom::Number(token.clone())
                    }
                    TokenType::String => Atom::String(token.clone()),
                    TokenType::Symbol => Atom::Symbol(token.clone()),
                    TokenType::Nil => Atom::Nil(token.clone()),
                    ttype => {
                        let (line_no, char_no) = self.previous_location();
                        return Err(Error::InvalidTypeForAtom {
                            ttype,
                            line_no,
                            char_no,
                        });
                    }
                }
            }
        };

        self.advance()?;

        Ok(atom)
    }
}
