use crate::{token::Token, visitor::Visitor};

#[derive(Debug, Clone)]
pub enum Expression {
    Define(Define),
    If(If),
    While(While),
    Lambda(Lambda),
    Call(Call),
    Datum(Datum),
}

impl Expression {
    pub fn accept<T: std::fmt::Debug>(
        &self,
        visitor: &mut dyn Visitor<T>,
    ) -> T {
        match self {
            Expression::Define(define) => visitor.visit_define(define),
            Expression::If(if_expr) => visitor.visit_if(if_expr),
            Expression::While(while_expr) => visitor.visit_while(while_expr),
            Expression::Lambda(lambda) => visitor.visit_lambda(lambda),
            Expression::Call(call) => visitor.visit_call(call),
            Expression::Datum(datum) => visitor.visit_datum(datum),
        }
    }

    pub fn node_type(&self) -> &str {
        match self {
            Expression::Define(_) => "def",
            Expression::If(_) => "if",
            Expression::While(_) => "while",
            Expression::Lambda(_) => "lambda",
            Expression::Call(_) => "call",
            Expression::Datum(_) => "datum",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Define {}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub then_branch: Box<Expression>,
    pub else_branch: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Box<Expression>,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Lambda {}

#[derive(Debug, Clone)]
pub struct Call {}

#[derive(Debug, Clone)]
pub enum Datum {
    List(Vec<Expression>),
    Symbol(Token),
}
