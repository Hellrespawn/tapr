use self::ast::*;
use crate::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::visitor::graph::DotVisitor;
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

    fn program(&mut self) -> Result<Program> {
        let expression = Box::new(self.expression()?);

        if self.current_token.is_some() {
            Err(self.error_at_current(ErrorKind::ParserError(
                "Program must contain a single expression.".to_owned(),
            )))
        } else {
            Ok(Program { expression })
        }
    }

    fn gather_expressions_until_paren(&mut self) -> Result<Vec<Node>> {
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

    fn expression(&mut self) -> Result<Node> {
        if let Some(TokenType::LeftParen) = self.current_type() {
            self.advance()?;

            let expression = match self.current_type().unwrap_or(TokenType::EOF)
            {
                TokenType::If => self.if_expression()?.into(),
                TokenType::While => self.while_expression()?.into(),
                TokenType::Set => self.set_expression()?.into(),
                TokenType::Def => self.function_definition()?.into(),
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

    fn single_variable(&mut self) -> Result<Variable> {
        let name = self.consume_symbol_token(
            "Set expression must be followed by a symbol.",
        )?;

        let node = Box::new(self.expression()?);

        Ok(Variable { name, node })
    }

    fn multiple_variables(&mut self) -> Result<Vec<Variable>> {
        let mut variables = Vec::new();

        loop {
            self.consume(
                TokenType::LeftParen,
                "Variable declaration must start with '('",
            )?;

            variables.push(self.single_variable()?);

            self.consume(
                TokenType::RightParen,
                "Variable declaration must end with ')'",
            )?;

            if self.matches(TokenType::RightParen) {
                break;
            }
        }

        Ok(variables)
    }

    fn gather_variables(&mut self) -> Result<Vec<Variable>> {
        if self.matches(TokenType::LeftParen) {
            self.multiple_variables()
        } else {
            Ok(vec![self.single_variable()?])
        }
    }

    fn set_expression(&mut self) -> Result<SetExpression> {
        self.consume(TokenType::Set, "Set Expression must start with 'set'")?;

        self.consume(
            TokenType::LeftParen,
            "Set Expression variables must start with '('",
        )?;

        let variables = self.gather_variables()?;

        self.consume(
            TokenType::RightParen,
            "Set Expression variables must end with ')'",
        )?;

        let scope = Box::new(self.expression()?);

        self.consume(
            TokenType::RightParen,
            "Set Expression must end with ')'",
        )?;

        Ok(SetExpression { variables, scope })
    }

    fn consume_symbol_token(&mut self, error_message: &str) -> Result<Token> {
        let Ok(Atom::Symbol(symbol)) = self.atom() else {

            return Err(
                self.error_at_previous(
                    ErrorKind::ParserError(
                        error_message.to_owned()
                    )
                )
            )
        };

        Ok(symbol)
    }

    fn parameter(&mut self) -> Result<Token> {
        self.consume_symbol_token("Function parameter must be a symbol.")
    }

    fn parameters(&mut self) -> Result<Vec<Token>> {
        if self.matches(TokenType::LeftParen) {
            self.advance()?;

            let mut parameters = Vec::new();

            while self.current_token.is_some()
                && !self.matches(TokenType::RightParen)
            {
                parameters.push(self.parameter()?);
            }

            self.consume(
                TokenType::RightParen,
                "Multiple function parameters must end with ')'",
            )?;

            Ok(parameters)
        } else {
            Ok(vec![self.parameter()?])
        }
    }

    fn function_definition(&mut self) -> Result<FunctionDefinition> {
        self.consume(
            TokenType::Def,
            "Function definition must start with 'def'",
        )?;

        let name = self.consume_symbol_token(
            "Function definition must have symbol for name.",
        )?;

        let parameters = self.parameters()?;

        let expression = Box::new(self.expression()?);

        self.consume(
            TokenType::RightParen,
            "Function definition must end with ')'",
        )?;

        let function_definition = FunctionDefinition {
            name,
            parameters,
            expression,
        };

        Ok(function_definition)
    }

    fn function_call(&mut self) -> Result<FunctionCall> {
        let name = self
            .consume_symbol_token("Function call must have symbol for name.")?;

        let arguments = self.gather_expressions_until_paren()?;

        self.advance()?;

        Ok(FunctionCall { name, arguments })
    }

    fn list(&mut self) -> Result<List> {
        let (line_no, col_no) = self.previous_location();

        let mut expressions: Vec<Node> = Vec::new();

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
            Err(Self::error(
                line_no,
                col_no,
                ErrorKind::ParserError(
                    "Quote must be followed by symbol.".to_owned(),
                ),
            ))
        }
    }

    fn atom(&mut self) -> Result<Atom> {
        let atom = match self.current_token.as_ref() {
            None => return Err(Error::without_location(ErrorKind::EmptyInput)),
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
                        return Err(self.error_at_previous(
                            ErrorKind::InvalidTypeForAtom(ttype),
                        ));
                    }
                }
            }
        };

        self.advance()?;

        Ok(atom)
    }
}
