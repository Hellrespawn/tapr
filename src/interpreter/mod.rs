use crate::parser::ast::{List, Program};
use crate::Result;

pub struct Interpreter {
    program: Program,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn interpret(&self) -> Result<()> {
        Interpreter::interpret_program(&self.program)
    }

    fn interpret_program(program: &Program) -> Result<()> {
        for list in &program.lists {
            Interpreter::interpret_list(list)?;
        }

        todo!()
    }

    fn interpret_list(list: &List) -> Result<()> {
        todo!()
    }
}
