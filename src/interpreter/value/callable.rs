use crate::interpreter::arguments::Arguments;
use crate::{Interpreter, Parameters, Result};

#[derive(Hash, PartialEq, Eq)]
pub enum CallableType {
    Native,
    Function,
    Macro,
}

impl std::fmt::Display for CallableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CallableType::Native => "native fn",
                CallableType::Function => "fn",
                CallableType::Macro => "macro",
            }
        )
    }
}

pub trait Callable<T>: Send + Sync {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Arguments<T>,
    ) -> Result<T>;

    fn arity(&self) -> usize;

    fn callable_type(&self) -> CallableType;

    fn parameters(&self) -> Parameters;
}

impl<T> std::fmt::Display for dyn Callable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}> ", self.callable_type(), self.parameters())
    }
}

impl<T> std::fmt::Debug for dyn Callable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
