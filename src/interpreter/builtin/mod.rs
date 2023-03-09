mod arithmetic;
mod boolean;
mod io;
mod list;

use super::{Interpreter, Value};
use crate::Result;

type BuiltinFunction =
    fn(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;

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
        write!(f, "builtin function {}", self.name)
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
        arguments: Vec<Value>,
    ) -> Result<Value> {
        (self.function)(intp, arguments)
    }
}

pub fn get_builtin_functions() -> Vec<Builtin> {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("+", arithmetic::add),
        ("-", arithmetic::sub),
        ("*", arithmetic::mul),
        ("/", arithmetic::div),
        ("%", arithmetic::modulus),
        ("!", boolean::not),
        (">", boolean::gt),
        (">=", boolean::gte),
        ("==", boolean::eq),
        ("<=", boolean::lte),
        ("<", boolean::lt),
        ("!=", boolean::ne),
        ("print", io::print),
        ("read", io::read),
        ("eval", io::eval),
        ("head", list::head),
        ("tail", list::tail),
        ("map", list::map),
        ("filter", list::filter),
    ];

    builtins
        .into_iter()
        .map(|(name, fun)| Builtin::new(name, fun))
        .collect()
}
