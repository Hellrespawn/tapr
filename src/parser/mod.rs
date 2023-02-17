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

    pub fn parse(&mut self) -> Result<Node> {
        self.advance()?;

        let program = self.program()?;

        let node: Node = program.into();

        if *DEBUG_AST {
            let filename = format!(
                "{}.{}.ast.dot",
                env!("CARGO_PKG_NAME"),
                self.parser_no
            );

            DotVisitor::create_ast_dot(&node, &filename);
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
            let (line_no, col_no) = self.previous_location();

            Err(Error::ConsumeError {
                message: message.to_owned(),
                line_no,
                col_no,
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

    fn program(&mut self) -> Result<Program> {
        let expression = Box::new(self.expression()?);

        if self.current_token.is_some() {
            let (line_no, col_no) = self.current_location();

            Err(Error::Parser {
                message: "Program must contain a single expression.".to_owned(),
                line_no,
                col_no,
            })
        } else {
            Ok(Program { expression })
        }
    }

    fn gather_expressions_until_paren(&mut self) -> Result<Vec<Node>> {
        let mut expressions = Vec::new();

        let (line_no, col_no) = self.previous_location();

        while !self.matches(TokenType::RightParen) {
            if self.current_token.is_none() {
                return Err(Error::UnmatchedParenthesis { line_no, col_no });
            }

            expressions.push(self.expression()?);
        }

        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Node> {
        if let Some(TokenType::LeftParen) = self.current_type() {
            self.advance()?;

            let expression = match self.current_type().unwrap_or(TokenType::EOF)
            {
                TokenType::If => self.if_expression()?.into(),
                TokenType::While => self.while_expression()?.into(),
                TokenType::Var => self.var_expression()?.into(),
                TokenType::Symbol => self.function_call()?.into(),
                _ => self.list()?.into(),
            };

            Ok(expression)
        } else {
            self.data_type()
        }
    }

    fn if_expression(&mut self) -> Result<IfExpression> {
        self.consume(TokenType::If, "First 'if'")?;

        let condition = Box::new(self.expression()?);

        let then_branch = Box::new(self.expression()?);

        let else_branch = if self.matches(TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.expression()?))
        };

        self.consume(TokenType::RightParen, "If Expression must end with ')'")?;

        Ok(IfExpression {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_expression(&mut self) -> Result<WhileExpression> {
        self.consume(
            TokenType::While,
            "While Expression must start with 'while'",
        )?;

        let condition = Box::new(self.expression()?);

        let then_branch = Box::new(self.expression()?);

        self.consume(
            TokenType::RightParen,
            "While Expression must end with ')'",
        )?;

        Ok(WhileExpression {
            condition,
            then_branch,
        })
    }

    fn var_expression(&mut self) -> Result<VarExpression> {
        self.consume(TokenType::Var, "Var Expression must start with 'var'")?;

        let Atom::Symbol(name) = self.atom()? else {
            let (line_no, col_no) = self.previous_location();

            return Err(
                Error::Parser{
                    message: "Var expression must be followed by a symbol.".to_owned(),
                    line_no,
                    col_no
                }
            );
        };

        let value = Box::new(self.expression()?);

        let scope = Box::new(self.expression()?);

        self.consume(
            TokenType::RightParen,
            "Var Expression must end with ')'",
        )?;

        Ok(VarExpression { name, value, scope })
    }

    fn function_call(&mut self) -> Result<FunctionCall> {
        let Atom::Symbol(symbol) = self.atom()? else {
            let (line_no, col_no) = self.previous_location();

            return Err(
                Error::Parser{
                    message: "Function call must be followed by a symbol.".to_owned(),
                    line_no,
                    col_no
                }
            );
        };

        let arguments = self.gather_expressions_until_paren()?;

        self.advance()?;

        Ok(FunctionCall {
            name: symbol,
            arguments,
        })
    }

    fn list(&mut self) -> Result<List> {
        let (line_no, col_no) = self.previous_location();

        let mut expressions: Vec<Node> = Vec::new();

        while !self.matches(TokenType::RightParen) {
            if self.current_token.is_none() {
                return Err(Error::UnmatchedParenthesis { line_no, col_no });
            }

            expressions.push(self.expression()?);
        }

        self.advance()?;

        Ok(List { expressions })
    }

    fn data_type(&mut self) -> Result<Node> {
        let node = if let Some(TokenType::Quote) = self.current_type() {
            self.quote()?.into()
        } else {
            self.atom()?.into()
        };

        Ok(node)
    }

    fn quote(&mut self) -> Result<FunctionCall> {
        self.advance()?;

        let (line_no, col_no) = self.previous_location();

        let argument = self.atom()?;

        if let Atom::Symbol(_) = &argument {
            let name = Token::new(
                TokenType::Symbol,
                "quote".to_owned(),
                line_no,
                col_no,
            );

            Ok(FunctionCall {
                name,
                arguments: vec![argument.into()],
            })
        } else {
            Err(Error::Parser {
                message: "Quote must be followed by symbol.".to_owned(),
                line_no,
                col_no,
            })
        }
    }

    fn atom(&mut self) -> Result<Atom> {
        let atom = match self.current_token.as_ref() {
            None => return Err(Error::EmptyInput),
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
                        let (line_no, col_no) = self.previous_location();
                        return Err(Error::InvalidTypeForAtom {
                            ttype,
                            line_no,
                            col_no,
                        });
                    }
                }
            }
        };

        self.advance()?;

        Ok(atom)
    }
}
