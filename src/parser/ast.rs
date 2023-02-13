use crate::interpreter::Visitor;

#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    List(List),
    Atom(Atom),
}

impl Node {
    pub fn accept<T: std::fmt::Debug>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Node::Program(program) => visitor.visit_program(program),
            Node::List(list) => visitor.visit_list(list),
            Node::Atom(atom) => visitor.visit_atom(atom),
        }
    }
}

impl From<Program> for Node {
    fn from(program: Program) -> Self {
        Self::Program(program)
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
    pub lists: Vec<List>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Number(f64),
    String(String),
    Symbol(String),
    Nil,
}
