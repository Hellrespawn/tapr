use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

type BooleanOp = fn(&Value, &Value) -> bool;

fn boolean_function(
    intp: &mut Interpreter,
    op: BooleanOp,
    argument_nodes: &[Expression],
) -> Result<Value> {
    let arguments =
        boolean_params().evaluate_arguments(intp, argument_nodes)?;

    let mut value = true;

    for window in arguments.windows(2) {
        let [lhs, rhs] = window else {
            unreachable!()
        };

        value = op(lhs, rhs);
    }

    Ok(Value::Boolean(value))
}

pub fn gt(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    boolean_function(intp, |lhs, rhs| rhs > lhs, argument_nodes)
}

pub fn gte(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    boolean_function(intp, |lhs, rhs| rhs >= lhs, argument_nodes)
}

pub fn eq(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    boolean_function(intp, |lhs, rhs| rhs == lhs, argument_nodes)
}

pub fn ne(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    Ok(Value::Boolean(eq(intp, argument_nodes)?.is_falsy()))
}

pub fn lte(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    boolean_function(intp, |lhs, rhs| rhs <= lhs, argument_nodes)
}

pub fn lt(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    boolean_function(intp, |lhs, rhs| rhs < lhs, argument_nodes)
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
