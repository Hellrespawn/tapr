use crate::{Interpreter, Node, Parameters, Result};

pub enum CallableType {
    Native,
    Function,
    Macro,
    SpecialForm,
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
                CallableType::SpecialForm => "special",
            }
        )
    }
}

pub trait Callable: Send + Sync {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Node>,
    ) -> Result<Node>;

    fn arity(&self) -> usize;

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
