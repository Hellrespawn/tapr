use super::{NativeFunction, NativeFunctionImpl, NativeMacro, NativeMacroImpl};
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
pub type NativeMacroTuple = (&'static str, NativeMacroImpl, &'static str);

pub trait NativeModule {
    fn environment(&self) -> Environment;

    fn name(&self) -> &'static str;

    fn is_core_module(&self) -> bool;
}

fn function_tuples_to_environment(
    environment: &mut Environment,
    tuples: Vec<NativeFunctionTuple>,
    name: &str,
) {
    for tuple in tuples {
        environment
            .def(tuple.0.to_owned(), function_tuple_to_value(tuple))
            .unwrap_or_else(|_| {
                panic!("Unable to add {name} functions to environment.")
            });
    }
}
fn macro_tuples_to_environment(
    environment: &mut Environment,
    tuples: Vec<NativeMacroTuple>,
    name: &str,
) {
    for tuple in tuples {
        environment
            .def(tuple.0.to_owned(), macro_tuple_to_value(tuple))
            .unwrap_or_else(|_| {
                panic!("Unable to add {name} functions to environment.")
            });
    }
}

fn function_tuple_to_value(tuple: NativeFunctionTuple) -> Value {
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

fn macro_tuple_to_value(tuple: NativeMacroTuple) -> Value {
    NativeMacro::new(
        tuple.0,
        tuple.1,
        tuple
            .2
            .try_into()
            .expect("Native macro should have valid parameters-string."),
    )
    .into()
}
