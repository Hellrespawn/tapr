use crate::interpreter::arguments::Arguments;
use crate::location::Location;
use crate::{Interpreter, Parameters, Result};

#[derive(Hash, PartialEq, Eq)]
pub enum CallableType {
    NativeFunction,
    NativeMacro,
    Function,
    Macro,
}

impl std::fmt::Display for CallableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CallableType::NativeFunction => "native fn",
                CallableType::NativeMacro => "native macro",
                CallableType::Function => "fn",
                CallableType::Macro => "macro",
            }
        )
    }
}

pub trait Callable<T>: Send + Sync {
    fn call(
        &self,
        location: Location,
        intp: &mut Interpreter,
        arguments: Arguments<T>,
    ) -> Result<T>;

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
