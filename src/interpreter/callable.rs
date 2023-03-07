use super::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

pub trait Callable: std::fmt::Debug {
    fn name(&self) -> &str;

    fn call(&self, intp: &mut Interpreter, arguments: &[Expression])
        -> Result<Value>;
}
