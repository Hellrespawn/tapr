use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

fn arithmetic(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    op: ArithmeticOp,
    intp: &mut Interpreter,
) -> Result<Value> {
    let arguments = parameters.evaluate_arguments(intp, argument_nodes)?;

    let mut iter = arguments.into_iter();

    let mut acc = iter.next().expect("at least one arguments");

    for rhs in iter {
        match op {
            // parameters here check for Numbers, so this is always safe.
            ArithmeticOp::Add => acc = (acc + rhs).unwrap(),
            ArithmeticOp::Sub => acc = (acc - rhs).unwrap(),
            ArithmeticOp::Mul => acc = (acc * rhs).unwrap(),
            ArithmeticOp::Div => acc = (acc / rhs).unwrap(),
        }
    }

    Ok(acc)
}

pub fn add(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    arithmetic(parameters, argument_nodes, ArithmeticOp::Add, intp)
}

pub fn sub(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    arithmetic(parameters, argument_nodes, ArithmeticOp::Sub, intp)
}

pub fn mul(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    arithmetic(parameters, argument_nodes, ArithmeticOp::Mul, intp)
}

pub fn div(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    arithmetic(parameters, argument_nodes, ArithmeticOp::Div, intp)
}

pub fn arithmetic_params() -> Parameters {
    let param = Parameter::new(
        "_arithmetic".to_owned(),
        vec![ParameterType::Number],
        true,
    );

    Parameters::new(vec![param]).expect("arithmetic to have valid params")
}
