mod arithmetic;
mod boolean;
mod io;
mod list;

use super::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

type BuiltinFunction =
    fn(intp: &mut Interpreter, argument_nodes: &[Expression]) -> Result<Value>;

#[derive(Clone)]
pub struct Builtin {
    name: &'static str,
    function: BuiltinFunction,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Builtin").field("name", &self.name).finish()
    }
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function {}>", self.name)
    }
}

impl Builtin {
    pub fn new(name: &'static str, function: BuiltinFunction) -> Self {
        Self { name, function }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Expression],
    ) -> Result<Value> {
        (self.function)(intp, argument_nodes)
    }
}

pub fn get_builtin_functions() -> Vec<Builtin> {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("+", arithmetic::add),
        ("-", arithmetic::sub),
        ("*", arithmetic::mul),
        ("/", arithmetic::div),
        (">", boolean::gt),
        (">=", boolean::gte),
        ("==", boolean::eq),
        ("<=", boolean::lte),
        ("<", boolean::lt),
        ("!=", boolean::ne),
        ("print", io::print),
        ("tail", list::tail),
        ("quote", io::quote),
    ];

    builtins
        .into_iter()
        .map(|(name, fun)| Builtin::new(name, fun))
        .collect()
}
