use crate::interpreter::{Arguments, Interpreter, Value};
use crate::parser::parameters::{Parameter, Parameters};
use crate::Result;

// type ArithmeticOp = fn(&str, &str) -> String;
type UnaryOp = fn(&str) -> String;

// fn arbitrary(op: ArithmeticOp, arguments: Vec<Value>) -> Result<Value> {
//     let params = arbitrary_params();

//     let arguments = Arguments::new(&params, arguments)?;

//     let strings = arguments.unwrap_strings();

//     let mut iter = strings.into_iter();

//     let mut acc = iter.next().expect("at least two arguments");

//     for rhs in iter {
//         acc = op(&acc, &rhs);
//     }

//     Ok(Value::String(acc))
// }

fn unary(op: UnaryOp, arguments: Vec<Value>) -> Result<Value> {
    let params = unary_params();
    let arguments = Arguments::new(&params, arguments)?;

    let string = arguments.unwrap_string(0);

    Ok(Value::String(op(&string)))
}

pub fn len(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = unary_params();
    let arguments = Arguments::new(&params, arguments)?;

    #[allow(clippy::cast_precision_loss)]
    Ok(Value::Number(arguments.unwrap_string(0).len() as f64))
}

pub fn join(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = join_params();

    let arguments = Arguments::new(&params, arguments)?;

    let separator = arguments.unwrap_string(0);

    let operands = arguments.unwrap_strings_from(1);

    Ok(Value::String(operands.join(&separator)))
}

pub fn join_not_nil(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    let params = join_params();

    let arguments = Arguments::new(&params, arguments)?;

    let separator = arguments.unwrap_string(0);

    let operands = arguments.unwrap_from(1);

    let strings = operands
        .into_iter()
        .filter_map(|v| {
            if let Value::String(string) = v {
                Some(string)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(Value::String(strings.join(&separator)))
}

pub fn trim(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    unary(|s| s.trim().to_owned(), arguments)
}

pub fn unary_params() -> Parameters {
    Parameter::empty("string".to_owned()).string().into()
}

// pub fn arbitrary_params() -> Parameters {
//     Parameters::new(vec![
//         Parameter::anonymous(vec![ParameterType::String], false),
//         Parameter::anonymous(vec![ParameterType::String], true),
//     ])
//     .expect("arithmetic to have valid params")
// }

fn join_params() -> Parameters {
    Parameters::new(vec![
        Parameter::empty("separator".to_owned()).string(),
        Parameter::empty("strings".to_owned()).string().rest(),
    ])
    .expect("join parameters to be valid.")
}
