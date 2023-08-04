#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]
use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::error::ErrorKind;
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, ParameterType, Result};

pub struct Debug;

impl NativeModule for Debug {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> =
            vec![("lsmod", lsmod, "&opt m:module")];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "debug"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn lsmod(
    _source: NodeSource,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    if let Some(argument) = arguments.get(0) {
        if let NodeData::Module(environment) = argument.data() {
            println!("{environment}");
        } else {
            return Err(ErrorKind::InvalidNodeArgument {
                expected: vec![ParameterType::Module],
                actual: argument,
            }
            .into());
        }
    } else {
        println!("{}", intp.environment());
    }

    Ok(Node::unknown(NodeData::Nil))
}
