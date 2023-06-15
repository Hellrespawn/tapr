use super::{arithmetic, NativeFunction};
use crate::interpreter::Parameters;

fn get_core_functions() -> Vec<NativeFunction> {
    let variadic_arithmetic_params: Parameters =
        "n:number & m:number".try_into().unwrap();

    vec![
        NativeFunction::new(
            "+",
            arithmetic::add,
            variadic_arithmetic_params.clone(),
        ),
        NativeFunction::new(
            "-",
            arithmetic::subtract,
            variadic_arithmetic_params.clone(),
        ),
        NativeFunction::new(
            "*",
            arithmetic::multiply,
            variadic_arithmetic_params.clone(),
        ),
        NativeFunction::new(
            "/",
            arithmetic::divide,
            variadic_arithmetic_params,
        ),
    ]
}
