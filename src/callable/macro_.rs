use crate::location::Location;
use crate::{
    Arguments, Callable, CallableType, Interpreter, Node, Parameters, Result,
};

pub type NativeMacroImpl = fn(
    location: Location,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node>;

#[derive(Debug, Clone)]
pub struct NativeMacro {
    name: &'static str,
    macro_: NativeMacroImpl,
    parameters: Parameters,
}

impl NativeMacro {
    pub fn new(
        name: &'static str,
        macro_: NativeMacroImpl,
        parameters: Parameters,
    ) -> Self {
        Self {
            name,
            macro_,
            parameters,
        }
    }
}

impl std::fmt::Display for NativeMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native macro {} {}>", self.name, self.parameters)
    }
}

impl Callable for NativeMacro {
    fn call(
        &self,
        location: Location,
        intp: &mut Interpreter,
        arguments: Arguments,
    ) -> Result<Node> {
        (self.macro_)(location, intp, arguments)
    }

    fn callable_type(&self) -> CallableType {
        CallableType::NativeMacro
    }

    fn parameters(&self) -> Parameters {
        self.parameters.clone()
    }
}
