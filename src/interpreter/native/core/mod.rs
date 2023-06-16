use super::{tuple_to_value, NativeFunctionTuple};
use crate::interpreter::environment::Environment;

mod arithmetic;
mod boolean;

pub fn get_core_environment() -> Environment {
    let tuples: Vec<NativeFunctionTuple> = vec![
        ("+", arithmetic::add, "n:number & m:number"),
        ("-", arithmetic::subtract, "n:number & m:number"),
        ("/", arithmetic::divide, "n:number & m:number"),
        ("*", arithmetic::multiply, "n:number & m:number"),
        ("%", arithmetic::modulus, "n:number m:number"),
        ("++", arithmetic::increment, "n:number"),
        ("--", arithmetic::decrement, "n:number"),
        ("!", boolean::not, "b:boolean"),
        (">", boolean::gt, "& b:boolean"),
        (">=", boolean::gte, "& b:boolean"),
        ("==", boolean::eq, "& b:boolean"),
        ("<=", boolean::lte, "& b:boolean"),
        ("<", boolean::lt, "& b:boolean"),
        ("!=", boolean::ne, "& b:boolean"),
    ];

    let mut environment = Environment::new();

    for tuple in tuples {
        environment
            .insert(tuple.0.to_owned(), tuple_to_value(tuple))
            .expect("Unable to add core functions to environment.");
    }

    environment
}
