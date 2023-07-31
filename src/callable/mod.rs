mod macro_;
mod native;
mod native_function;
mod function;

use crate::arguments::Arguments;
use crate::location::Location;
use crate::{Interpreter, Parameters, Result, Node};

pub use macro_::{NativeMacro, NativeMacroImpl};
pub use native_function::{NativeFunction, NativeFunctionImpl};
pub use native::get_default_environment;
pub use function::Function;

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

pub trait Callable: Send + Sync {
    fn call(
        &self,
        location: Location,
        intp: &mut Interpreter,
        arguments: Arguments,
    ) -> Result<Node>;

    fn callable_type(&self) -> CallableType;

    fn parameters(&self) -> Parameters;
}

impl std::fmt::Display for dyn Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}> ", self.callable_type(), self.parameters())
    }
}

impl std::fmt::Debug for dyn Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
