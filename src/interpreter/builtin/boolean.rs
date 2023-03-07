use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub enum BooleanOp {
    Greater,
    GreaterEqual,
    Equal,
    LessEqual,
    Less,
    NotEqual,
}

fn boolean_function(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    op: BooleanOp,
    intp: &mut Interpreter,
) -> Result<Value> {
    let arguments = parameters.evaluate_arguments(intp, argument_nodes)?;

    let mut value = true;

    for window in arguments.windows(2) {
        let [lhs, rhs] = window else {
            unreachable!()
        };

        value = match op {
            BooleanOp::Greater => lhs > rhs,
            BooleanOp::GreaterEqual => lhs >= rhs,
            BooleanOp::Equal => lhs == rhs,
            BooleanOp::LessEqual => lhs <= rhs,
            BooleanOp::Less => lhs < rhs,
            BooleanOp::NotEqual => lhs != rhs,
        }
    }

    Ok(Value::Boolean(value))
}

pub fn gt(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::Greater, intp)
}

pub fn gte(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::GreaterEqual, intp)
}

pub fn eq(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::Equal, intp)
}

pub fn lte(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::LessEqual, intp)
}

pub fn lt(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::Less, intp)
}

pub fn ne(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    boolean_function(parameters, argument_nodes, BooleanOp::NotEqual, intp)
}

pub fn boolean_params() -> Parameters {
    let param = Parameter::new(
        "_arithmetic".to_owned(),
        vec![ParameterType::Any],
        false,
    );

    let remaining_params = Parameter::new(
        "_arithmetic".to_owned(),
        vec![ParameterType::Any],
        true,
    );

    Parameters::new(vec![param, remaining_params])
        .expect("arithmetic to have valid params")
}
