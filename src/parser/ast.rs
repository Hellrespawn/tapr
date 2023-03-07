use crate::token::Token;
use crate::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    IfExpression(IfExpression),
    WhileExpression(WhileExpression),
    SetExpression(SetExpression),
    FunctionCall(FunctionCall),
    FunctionDefinition(FunctionDefinition),
    List(List),
    Atom(Atom),
}

impl Node {
    pub fn accept<T: std::fmt::Debug>(
        &self,
        visitor: &mut dyn Visitor<T>,
    ) -> T {
        match self {
            Node::Program(program) => visitor.visit_program(program),
            Node::IfExpression(if_expression) => {
                visitor.visit_if_expression(if_expression)
            }
            Node::WhileExpression(while_expression) => {
                visitor.visit_while_expression(while_expression)
            }
            Node::SetExpression(set_expression) => {
                visitor.visit_set_expression(set_expression)
            }
            Node::FunctionCall(function_call) => {
                visitor.visit_function_call(function_call)
            }
            Node::FunctionDefinition(function_definition) => {
                visitor.visit_function_definition(function_definition)
            }
            Node::List(list) => visitor.visit_list(list),
            Node::Atom(atom) => visitor.visit_atom(atom),
        }
    }

    pub fn node_type(&self) -> &str {
        match self {
            Node::Program(_) => "Program",
            Node::IfExpression(_) => "IfExpression",
            Node::WhileExpression(_) => "WhileExpression",
            Node::SetExpression(_) => "SetExpression",
            Node::FunctionCall(_) => "FunctionCall",
            Node::FunctionDefinition(_) => "FunctionDefinition",
            Node::List(_) => "List",
            Node::Atom(_) => "Atom",
        }
    }
}

impl From<Program> for Node {
    fn from(program: Program) -> Self {
        Self::Program(program)
    }
}

impl From<IfExpression> for Node {
    fn from(if_expression: IfExpression) -> Self {
        Self::IfExpression(if_expression)
    }
}

impl From<WhileExpression> for Node {
    fn from(while_expression: WhileExpression) -> Self {
        Self::WhileExpression(while_expression)
    }
}

impl From<SetExpression> for Node {
    fn from(set_expression: SetExpression) -> Self {
        Self::SetExpression(set_expression)
    }
}

impl From<FunctionCall> for Node {
    fn from(function_call: FunctionCall) -> Self {
        Self::FunctionCall(function_call)
    }
}

impl From<FunctionDefinition> for Node {
    fn from(function_definition: FunctionDefinition) -> Self {
        Self::FunctionDefinition(function_definition)
    }
}

impl From<List> for Node {
    fn from(list: List) -> Self {
        Self::List(list)
    }
}

impl From<Atom> for Node {
    fn from(atom: Atom) -> Self {
        Self::Atom(atom)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub expression: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Node>,
    pub then_branch: Box<Node>,
    pub else_branch: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct WhileExpression {
    pub condition: Box<Node>,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct SetExpression {
    pub name: Token,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: Token,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub expressions: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Boolean(Token),
    Number(Token),
    String(Token),
    Symbol(Token),
    Nil(Token),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
    pub node: Box<Node>,
}
