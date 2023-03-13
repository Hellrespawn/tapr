use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type BooleanOp = fn(&Value, &Value) -> bool;

fn boolean_function(op: BooleanOp, arguments: Vec<Value>) -> Result<Value> {
    let params = boolean_params();
    let arguments = Arguments::new(&params, arguments)?;

    for window in arguments.arguments().windows(2) {
        let [lhs, rhs] = window else {
            unreachable!()
        };

        // Short-circuit and return false if the condition is false.
        if !op(lhs, rhs) {
            return Ok(Value::Boolean(false));
        }
    }

    // Return true at the end.
    Ok(Value::Boolean(true))
}

pub fn gt(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    boolean_function(|lhs, rhs| rhs > lhs, arguments)
}

pub fn gte(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    boolean_function(|lhs, rhs| rhs >= lhs, arguments)
}

pub fn eq(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    boolean_function(|lhs, rhs| rhs == lhs, arguments)
}

pub fn ne(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    Ok(Value::Boolean(eq(intp, arguments)?.is_falsy()))
}

pub fn lte(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    boolean_function(|lhs, rhs| rhs <= lhs, arguments)
}

pub fn lt(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    boolean_function(|lhs, rhs| rhs < lhs, arguments)
}

pub fn not(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::Any, false).into();

    let arguments = Arguments::new(&params, arguments)?;

    let argument = arguments.unwrap(0);

    Ok(Value::Boolean(argument.is_falsy()))
}

pub fn boolean_params() -> Parameters {
    let param = Parameter::anonymous(ParameterType::Any, false);

    let remaining_params = Parameter::anonymous(ParameterType::Any, true);

    Parameters::new(vec![param, remaining_params])
        .expect("arithmetic to have valid params")
}
