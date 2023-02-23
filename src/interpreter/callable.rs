use super::{Interpreter, Value};
use crate::parser::ast::Node;
use crate::Result;

pub trait Callable: std::fmt::Debug {
    fn name(&self) -> &str;

    fn call(&self, intp: &mut Interpreter, arguments: &[Node])
        -> Result<Value>;
}
