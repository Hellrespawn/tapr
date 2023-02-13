mod builtin;

use self::builtin::{ArithmeticFunction, ArithmeticKind};
use super::Value;
use crate::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub trait Function: Sync + Send {
    fn call(&self, args: &[Value]) -> Result<Value>;
}

pub static BUILTIN_FUNCTIONS: Lazy<HashMap<&str, Box<dyn Function>>> =
    Lazy::new(|| {
        let mut map: HashMap<&str, Box<dyn Function>> = HashMap::new();

        map.insert("+", Box::new(ArithmeticFunction::new(ArithmeticKind::Add)));

        map.insert(
            "-",
            Box::new(ArithmeticFunction::new(ArithmeticKind::Subtract)),
        );

        map.insert(
            "*",
            Box::new(ArithmeticFunction::new(ArithmeticKind::Multiply)),
        );

        map.insert(
            "/",
            Box::new(ArithmeticFunction::new(ArithmeticKind::Divide)),
        );

        map
    });
