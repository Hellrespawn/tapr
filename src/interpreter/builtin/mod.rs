mod arithmetic;
mod boolean;
mod io;
mod list;

use super::callable::Callable;
use super::parameters::Parameters;
use super::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

type InnerBuiltinFunction = fn(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value>;

pub struct BuiltinFunction {
    name: &'static str,
    function: InnerBuiltinFunction,
    parameters: Parameters,
}

impl std::fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .finish()
    }
}

impl BuiltinFunction {
    pub fn new(
        name: &'static str,
        function: fn(
            parameters: &Parameters,
            argument_nodes: &[Expression],
            intp: &mut Interpreter,
        ) -> Result<Value>,
        parameters: Parameters,
    ) -> Self {
        Self {
            name,
            function,
            parameters,
        }
    }
}

impl Callable for BuiltinFunction {
    fn name(&self) -> &str {
        self.name
    }

    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Expression],
    ) -> Result<Value> {
        let function = self.function;

        function(&self.parameters, argument_nodes, intp)
    }
}

pub fn get_builtin_functions() -> Vec<BuiltinFunction> {
    let builtins: Vec<(&str, InnerBuiltinFunction, Parameters)> = vec![
        ("+", arithmetic::add, arithmetic::arithmetic_params()),
        ("-", arithmetic::sub, arithmetic::arithmetic_params()),
        ("*", arithmetic::mul, arithmetic::arithmetic_params()),
        ("/", arithmetic::div, arithmetic::arithmetic_params()),
        (">", boolean::gt, boolean::boolean_params()),
        (">=", boolean::gte, boolean::boolean_params()),
        ("==", boolean::eq, boolean::boolean_params()),
        ("<=", boolean::lte, boolean::boolean_params()),
        ("<", boolean::lt, boolean::boolean_params()),
        ("!=", boolean::ne, boolean::boolean_params()),
        ("print", io::print, io::print_params()),
        ("tail", list::tail, list::tail_params()),
        ("quote", io::quote, io::quote_params()),
    ];

    builtins
        .into_iter()
        .map(|(name, fun, params)| BuiltinFunction::new(name, fun, params))
        .collect()
}
