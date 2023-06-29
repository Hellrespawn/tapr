use super::{NativeFunction, NativeFunctionImpl};
use crate::interpreter::environment::Environment;
use crate::interpreter::Value;

mod arithmetic;
mod boolean;
mod core;
mod debug;
mod fs;
mod io;
mod list;
mod number;
mod string;

pub fn get_modules<'a>() -> Vec<&'a dyn NativeModule> {
    vec![
        &core::Core,
        &arithmetic::Arithmetic,
        &boolean::Boolean,
        &debug::Debug,
        &fs::Fs,
        &io::Io,
        &list::List,
        &number::Number,
        &string::StringModule,
    ]
}

pub type NativeFunctionTuple = (&'static str, NativeFunctionImpl, &'static str);

pub trait NativeModule {
    fn environment(&self) -> Environment;

    fn name(&self) -> &'static str;

    fn is_core_module(&self) -> bool;
}

fn tuples_to_environment(
    tuples: Vec<NativeFunctionTuple>,
    name: &str,
) -> Environment {
    let mut environment = Environment::new();

    for tuple in tuples {
        environment
            .def(tuple.0.to_owned(), tuple_to_value(tuple))
            .unwrap_or_else(|_| {
                panic!("Unable to add {name} functions to environment.")
            });
    }

    environment
}

fn tuple_to_value(tuple: NativeFunctionTuple) -> Value {
    NativeFunction::new(
        tuple.0,
        tuple.1,
        tuple
            .2
            .try_into()
            .expect("Native function should have valid parameters-string."),
    )
    .into()
}
