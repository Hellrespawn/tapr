use super::{Interpreter, Value};
use crate::error::{Error, ErrorKind};
use crate::parser::ast::Expression;
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub enum ParameterType {
    List,
    Number,
    String,
    Boolean,
    Symbol,
    Any,
}

impl ParameterType {
    fn value_is_type(self, value: &Value) -> bool {
        match self {
            ParameterType::List => matches!(value, Value::List(_)),
            ParameterType::Number => matches!(value, Value::Number(_)),
            ParameterType::String => matches!(value, Value::String(_)),
            ParameterType::Boolean => matches!(value, Value::Boolean(_)),
            ParameterType::Symbol => matches!(value, Value::Symbol(_)),
            ParameterType::Any => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    ptypes: Vec<ParameterType>,
    // TODO Implement optional parameters
    // is_required: bool
    is_variadic: bool,
}

impl Parameter {
    pub fn new(
        name: String,
        ptypes: Vec<ParameterType>,
        is_variadic: bool,
    ) -> Self {
        Self {
            name,
            ptypes,
            is_variadic,
        }
    }

    fn value_is_type(&self, value: &Value) -> Result<()> {
        if self.ptypes.iter().any(|ptype| ptype.value_is_type(value)) {
            Ok(())
        } else {
            Err(Error::without_location(ErrorKind::InvalidArgument {
                expected: self.ptypes.clone(),
                actual: value.clone(),
            }))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
}

impl Parameters {
    pub fn new(parameters: Vec<Parameter>) -> Result<Self> {
        let has_variadic_param_before_last = parameters
            .iter()
            .enumerate()
            .any(|(i, param)| param.is_variadic && i != parameters.len() - 1);

        if has_variadic_param_before_last {
            return Err(Error::without_location(
                ErrorKind::NonLastParameterIsVariadic,
            ));
        }

        Ok(Self { parameters })
    }

    pub fn evaluate_arguments(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Expression],
    ) -> Result<Vec<Value>> {
        self.check_amount_of_args_or_error(argument_nodes.len())?;

        let arguments = Self::interpret_arguments(intp, argument_nodes)?;

        self.check_types_of_args_or_error(&arguments)?;

        Ok(arguments)
    }

    pub fn check_amount_of_args_or_error(
        &self,
        number_of_arguments: usize,
    ) -> Result<()> {
        if !self.check_amount_of_args(number_of_arguments) {
            return Err(if self.is_variadic() {
                Error::without_location(ErrorKind::WrongAmountOfMinArgs {
                    expected: self.parameters.len(),
                    actual: number_of_arguments,
                })
            } else {
                Error::without_location(ErrorKind::WrongAmountOfFixedArgs {
                    expected: self.parameters.len(),
                    actual: number_of_arguments,
                })
            });
        }

        Ok(())
    }

    fn is_variadic(&self) -> bool {
        self.parameters.iter().any(|p| p.is_variadic)
    }

    fn check_amount_of_args(&self, number_of_arguments: usize) -> bool {
        let min = self.parameters.len();

        if self.is_variadic() {
            number_of_arguments >= min
        } else {
            number_of_arguments == min
        }
    }

    fn interpret_arguments(
        intp: &mut Interpreter,
        argument_nodes: &[Expression],
    ) -> Result<Vec<Value>> {
        argument_nodes
            .iter()
            .map(|node| node.accept(intp))
            .collect::<Result<Vec<_>>>()
    }

    fn check_types_of_args_or_error(&self, arguments: &[Value]) -> Result<()> {
        // Rust zips shortest, so we can always compare the fixed args
        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            parameter.value_is_type(argument)?;
        }

        if self.is_variadic() && arguments.len() > self.parameters.len() {
            // arguments.len() > self.params.len()
            let last_param =
                self.parameters.last().expect("at least one param");

            debug_assert!(last_param.is_variadic);

            // Compare remaing arguments to final (variadic) param
            let remainder = &arguments[self.parameters.len()..];

            for value in remainder {
                last_param.value_is_type(value)?;
            }
        }

        Ok(())
    }
}
