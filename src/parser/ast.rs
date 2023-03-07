use crate::token::Token;
use crate::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum Expression {
    Define(Define),
    If(If),
    While(While),
    Lambda(Lambda),
    Call(Call),
    QuotedDatum(Datum),
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
            Expression::QuotedDatum(datum) => visitor.visit_quoted_datum(datum),
            Expression::Datum(datum) => visitor.visit_datum(datum),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Define {
    pub name: Symbol,
    pub expression: Box<Expression>,
}

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
pub struct Lambda {
    pub parameters: Vec<Symbol>,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub symbol: Symbol,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Datum {
    List(List),
    Boolean(Boolean),
    Number(Number),
    String(StringNode),
    Symbol(Symbol),
    Nil,
}

#[derive(Debug, Clone)]
pub struct List {
    pub start_token: Token,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Boolean(pub Token);

#[derive(Debug, Clone)]
pub struct Number(pub Token);

#[derive(Debug, Clone)]
pub struct StringNode(pub Token);

#[derive(Debug, Clone)]
pub struct Symbol(pub Token);
