#[derive(Debug)]
pub enum Node {
    ProgramNode(Program),
    ListNode(List),
    AtomNode(Atom),
}

impl From<Program> for Node {
    fn from(program: Program) -> Self {
        Self::ProgramNode(program)
    }
}

impl From<List> for Node {
    fn from(list: List) -> Self {
        Self::ListNode(list)
    }
}

impl From<Atom> for Node {
    fn from(atom: Atom) -> Self {
        Self::AtomNode(atom)
    }
}

#[derive(Debug)]
pub struct Program {
    pub lists: Vec<List>,
}

#[derive(Debug)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Debug)]
pub enum Atom {
    Number(f64),
    String(String),
    Symbol(String),
}
