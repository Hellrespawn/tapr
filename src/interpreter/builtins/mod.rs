use super::environment::Environment;
use super::value::Builtin;
use super::{Interpreter, Value};
use crate::Result;
use once_cell::sync::Lazy;

mod arithmetic;
mod boolean;
mod debug;
mod file;
mod io;
mod list;
mod string;

pub type BuiltinFunction =
    fn(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;

static BUILTIN_ENVIRONMENT: Lazy<Environment> = Lazy::new(|| {
    let mut core_env = get_core_environment();

    let modules = vec![
        ("debug", get_debug_environment()),
        ("file", get_file_environment()),
        ("io", get_io_environment()),
        ("list", get_list_environment()),
        ("number", get_number_environment()),
        ("string", get_string_environment()),
    ];

    for (name, environment) in modules {
        core_env
            .insert(
                name.to_owned(),
                Value::Module {
                    prefix: name.to_owned(),
                    environment,
                },
            )
            .unwrap_or_else(|e| panic!("Unable to add module '{name}':\n{e}"));
    }

    core_env
});

pub fn get_builtin_environment() -> Environment {
    BUILTIN_ENVIRONMENT.clone()
}

fn get_core_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("+", arithmetic::add),
        // ("+=", arithmetic::add_assign), // TODO Op Assign
        ("-", arithmetic::subtract),
        ("*", arithmetic::multiply),
        ("/", arithmetic::divide),
        ("%", arithmetic::modulus),
        ("++", arithmetic::increment),
        ("--", arithmetic::decrement),
        ("!", boolean::not),
        (">", boolean::gt),
        (">=", boolean::gte),
        ("==", boolean::eq),
        ("<=", boolean::lte),
        ("<", boolean::lt),
        ("!=", boolean::ne),
        ("print", io::print),
        ("println", io::println),
    ];

    add_functions_to_environment(builtins)
}

fn get_debug_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![("env", debug::env)];

    add_functions_to_environment(builtins)
}

fn get_file_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> =
        vec![("read", file::read), ("write", file::write)];

    add_functions_to_environment(builtins)
}

fn get_io_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> =
        vec![("read", io::read), ("eval", io::eval)];

    add_functions_to_environment(builtins)
}

fn get_list_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("list", list::list),
        ("head", list::head),
        ("tail", list::tail),
        ("pop", list::push),
        ("peek", list::peek),
        // ("pop", list::pop),  // TODO Pop end of list
        // ("concat", list::concat),  // TODO Concatenate multiple lists
        // ("insert", list::insert),  // TODO Insert at index
        // ("remove", list::remove),  // TODO Remove at index
        // ("slice", list::slice),    // TODO Return slice
        ("reduce", list::reduce),
        ("map", list::map),
        ("filter", list::filter),
    ];

    add_functions_to_environment(builtins)
}

fn get_number_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> =
        vec![("parse-number", io::parse_number)];

    let mut env = add_functions_to_environment(builtins);

    env.insert("pi".to_owned(), Value::Number(std::f64::consts::PI))
        .unwrap();

    env
}

fn get_string_environment() -> Environment {
    let builtins: Vec<(&str, BuiltinFunction)> = vec![
        ("len", string::len),
        ("join", string::join),
        ("join-not-nil", string::join_not_nil),
        ("trim", string::trim),
        // ("format", string::format),  // TODO? printf style formatting?
        // ("lower", string::lower),  // TODO
        // ("upper", string::upper),  // TODO
        // ("starts-with", string::starts_with),  // TODO
        // ("ends-with", string::ends_with),  // TODO
        // ("repeat", string::repeat),  // TODO
        // ("replace", string::replace),  // TODO
        // ("slice", string::slice),  // TODO
        // ("split", string::split),  // TODO
    ];

    add_functions_to_environment(builtins)
}

fn add_functions_to_environment(
    functions: Vec<(&'static str, BuiltinFunction)>,
) -> Environment {
    let mut env = Environment::new();

    for (name, function) in functions {
        env.insert(
            name.to_owned(),
            Value::Builtin(Builtin::new(name, function)),
        )
        .unwrap_or_else(|e| {
            panic!("Unable to add '{name}' to default environment:\n{e}")
        });
    }

    env
}
