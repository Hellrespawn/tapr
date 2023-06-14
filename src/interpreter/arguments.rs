use super::environment::Environment;
use super::{Parameters, Value};
use crate::error::ErrorKind;
use crate::Result;

pub struct Arguments<'a> {
    parameters: &'a Parameters,
    arguments: Vec<Value>,
}

impl<'a> Arguments<'a> {
    pub fn new(
        parameters: &'a Parameters,
        arguments: Vec<Value>,
    ) -> Result<Self> {
        let arguments = Self {
            parameters,
            arguments,
        };

        arguments.check_length()?;
        arguments.check_types()?;

        Ok(arguments)
    }

    pub fn add_to_env(self, env: &mut Environment) -> Result<()> {
        for (parameter, argument) in self.parameters.iter().zip(self.arguments)
        {
            env.insert(parameter.name().unwrap().to_owned(), argument)?;
        }

        Ok(())
    }

    pub fn arguments(&self) -> &[Value] {
        &self.arguments
    }

    pub fn unwrap(&self, index: usize) -> Value {
        self.arguments
            .get(index)
            .cloned()
            .expect("Called unwrap on invalid index.")
    }

    pub fn unwrap_from(&self, index: usize) -> Vec<Value> {
        self.arguments[index..].to_vec()
    }

    pub fn unwrap_string(&self, index: usize) -> String {
        let argument = &self.arguments[index];

        let Value::String(string) = argument else {
            panic!("Called unwrap_string on non-Value::String")
        };

        string.clone()
    }

    pub fn unwrap_strings(&self) -> Vec<String> {
        self.unwrap_strings_from(0)
    }

    pub fn unwrap_strings_from(&self, start_index: usize) -> Vec<String> {
        self.arguments[start_index..]
            .iter()
            .map(|v| {
                let Value::String(string) = v else {
                panic!("Called unwrap_strings on non-Value::String")
            };
                string.clone()
            })
            .collect()
    }

    pub fn unwrap_list(&self, index: usize) -> Vec<Value> {
        let argument = &self.arguments[index];

        let Value::List(list) = argument else {
            panic!("Called unwrap_list on non-Value::List")
        };

        list.clone()
    }

    pub fn unwrap_function(&self, index: usize) -> Value {
        let argument = &self.arguments[index];

        if !matches!(argument, Value::Function(_) | Value::Builtin(_)) {
            panic!("Called unwrap_function on non-Value::{{Builtin, Function}}")
        }

        argument.clone()
    }

    pub fn unwrap_numbers(&self) -> Vec<f64> {
        self.arguments
            .iter()
            .map(|v| {
                let Value::Number(number) = v else {
                panic!("Called unwrap_numbers on non-Value::Number")
            };
                *number
            })
            .collect()
    }

    fn check_length(&self) -> Result<()> {
        if self.parameters.has_rest_param() {
            if self.parameters.len() > self.arguments.len() {
                Err(ErrorKind::WrongAmountOfMinArgs {
                    expected: self.parameters.len(),
                    actual: self.arguments.len(),
                }
                .into())
            } else {
                Ok(())
            }
        } else if self.parameters.len() != self.arguments.len() {
            Err(ErrorKind::WrongAmountOfFixedArgs {
                expected: self.parameters.len(),
                actual: self.arguments.len(),
            }
            .into())
        } else {
            Ok(())
        }
    }

    fn check_types(&self) -> Result<()> {
        // Check pairs of params and args, length is already checked here.
        for (param, arg) in self.parameters.iter().zip(&self.arguments) {
            param.value_is_type(arg)?;
        }

        // If function has a rest parameter...
        if self.parameters.has_rest_param() {
            let remaining_args = self.arguments.get(self.parameters.len()..);

            // and there are more args...
            if let Some(remaining_args) = remaining_args {
                let last_param = self.parameters.last().expect("Parameters should have a Parameter if has_rest_param is true.");

                // Check the last param.
                for arg in remaining_args {
                    last_param.value_is_type(arg)?;
                }
            }
        }

        Ok(())
    }
}
