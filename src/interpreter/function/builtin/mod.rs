mod arithmetic;
mod boolean;

pub use arithmetic::*;
pub use boolean::*;

use super::{Arguments, Function};
use crate::interpreter::Value;
use crate::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

struct PrintFunction;

impl PrintFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for PrintFunction {
    fn call(&self, args: &[Value]) -> Result<Value> {
        PrintFunction::ARGUMENTS.check_amount(args.len())?;

        println!("{}", &args[0]);

        Ok(Value::Nil)
    }
}

pub static BUILTIN_FUNCTIONS: Lazy<HashMap<&str, Box<dyn Function>>> =
    Lazy::new(|| {
        let mut map: HashMap<&str, Box<dyn Function>> = HashMap::new();

        map.insert(
            "+",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Add, 2)),
        );

        map.insert(
            "-",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Subtract, 2)),
        );

        map.insert(
            "*",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Multiply, 2)),
        );

        map.insert(
            "/",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Divide, 2)),
        );

        map.insert(">", Box::new(BooleanFunction::new(BooleanOp::Greater, 2)));
        map.insert(
            ">=",
            Box::new(BooleanFunction::new(BooleanOp::GreaterOrEqual, 2)),
        );
        map.insert("==", Box::new(BooleanFunction::new(BooleanOp::Equal, 2)));
        map.insert(
            "<=",
            Box::new(BooleanFunction::new(BooleanOp::LessOrEqual, 2)),
        );
        map.insert("<", Box::new(BooleanFunction::new(BooleanOp::Less, 2)));

        map.insert("print", Box::new(PrintFunction));

        map
    });
