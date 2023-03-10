use super::value::Builtin;
use super::{Interpreter, Value};
use crate::Result;

mod arithmetic;
mod boolean;
mod debug;
mod io;
mod list;

pub type BuiltinFunction =
    fn(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;

pub fn get_builtin_functions() -> Vec<Builtin> {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("_env", debug::env),
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
        ("list", list::list),
        ("head", list::head),
        ("tail", list::tail),
        ("last", list::last),
        ("concat", list::concat),
        ("reduce", list::reduce),
        ("map-native", list::map),
        ("filter-native", list::filter),
    ];

    builtins
        .into_iter()
        .map(|(name, fun)| Builtin::new(name, fun))
        .collect()
}
