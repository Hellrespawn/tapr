#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]

use super::{
    function_tuples_to_environment, macro_tuples_to_environment,
    NativeFunctionTuple, NativeMacroTuple, NativeModule,
};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::location::Location;
use crate::{Node, NodeData, Result};

pub struct Core;

impl NativeModule for Core {
    fn environment(&self) -> Environment {
        let function_tuples: Vec<NativeFunctionTuple> = vec![
            ("println", println, "& s"),
            ("print", print, "& s"),
            ("is-nil", is_nil, "v"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, function_tuples, self.name());

        let macro_tuples: Vec<NativeMacroTuple> =
            vec![("cond", cond, "& pairs")];

        macro_tuples_to_environment(&mut env, macro_tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "core"
    }

    fn is_core_module(&self) -> bool {
        true
    }
}

fn println(
    _location: Location,
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    println!();

    Ok(Value::nil())
}

fn print(
    _location: Location,
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    Ok(Value::nil())
}

fn is_nil(
    _location: Location,
    _: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let argument = arguments.unwrap(0);

    Ok(Value::bool(argument.is_nil()))
}

fn cond(
    location: Location,
    _: &mut Interpreter,
    arguments: Arguments<Node>,
) -> Result<Node> {
    let mut arguments = arguments.unwrap_from(0);

    let mut node = if arguments.len() % 2 == 1 {
        arguments.pop().unwrap()
    } else {
        Node::new(location, NodeData::Nil)
    };

    for pair in arguments.rchunks(2) {
        node = Node::new(
            pair[0].location(),
            NodeData::PTuple(vec![
                Node::new(
                    pair[0].location(),
                    NodeData::Symbol("if".to_owned()),
                ),
                pair[0].clone(),
                pair[1].clone(),
                node,
            ]),
        );
    }

    Ok(node)
}
