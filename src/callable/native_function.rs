use crate::node::NodeSource;
use crate::{
    Arguments, Callable, CallableType, Interpreter, Node, Parameters, Result,
};

pub type NativeFunctionImpl = fn(
    source: NodeSource,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node>;

#[derive(Debug, Clone)]
pub struct NativeFunction {
    name: &'static str,
    function: NativeFunctionImpl,
    parameters: Parameters,
}

impl NativeFunction {
    pub fn new(
        name: &'static str,
        function: NativeFunctionImpl,
        parameters: Parameters,
    ) -> Self {
        Self {
            name,
            function,
            parameters,
        }
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function {} {}>", self.name, self.parameters)
    }
}

impl Callable for NativeFunction {
    fn call(
        &self,
        source: NodeSource,
        intp: &mut Interpreter,
        arguments: Arguments,
    ) -> Result<Node> {
        (self.function)(source, intp, arguments)
    }

    fn callable_type(&self) -> CallableType {
        CallableType::NativeFunction
    }

    fn parameters(&self) -> Parameters {
        self.parameters.clone()
    }
}
