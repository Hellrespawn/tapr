use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

use super::{tuple_to_value, NativeFunctionTuple};

fn env(intp: &mut Interpreter, _arguments: &Arguments) -> Result<Value> {
    writeln!(intp.output, "{}", intp.environment)?;

    Ok(Value::Nil)
}

fn lsmod(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let environment = arguments.unwrap_module(0);

    writeln!(intp.output, "{environment}")?;

    Ok(Value::Nil)
}

pub fn get_debug_environment() -> Environment {
    let tuples: Vec<NativeFunctionTuple> =
        vec![("env", env, ""), ("lsmod", lsmod, "m:module")];

    let mut environment = Environment::new();

    for tuple in tuples {
        environment
            .insert(tuple.0.to_owned(), tuple_to_value(tuple))
            .expect("Unable to add core functions to environment.");
    }

    environment
}
