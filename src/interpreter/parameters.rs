use super::Value;
use crate::error::{Error, ErrorKind};
use crate::Result;

// TODO Optional parameters

#[derive(Debug, Clone)]
pub enum ParameterType {
    TypedList(Box<ParameterType>),
    List,
    Module,
    Function,
    Number,
    String,
    Boolean,
    Symbol,
    Keyword,
    Nil,
}

impl ParameterType {
    fn value_is_type(&self, value: &Value) -> bool {
        match self {
            ParameterType::Module => {
                matches!(value, Value::Module { .. })
            }
            ParameterType::Function => {
                matches!(value, Value::Function(_) | Value::Builtin(_))
            }
            ParameterType::List => matches!(value, Value::List(_)),
            ParameterType::Number => matches!(value, Value::Number(_)),
            ParameterType::String => matches!(value, Value::String(_)),
            ParameterType::Boolean => matches!(value, Value::Boolean(_)),
            ParameterType::Symbol => matches!(value, Value::Symbol(_)),
            ParameterType::Keyword => matches!(value, Value::Keyword(_)),
            ParameterType::Nil => matches!(value, Value::Nil),
            ParameterType::TypedList(ptype) => {
                if let Value::List(values) = value {
                    values.iter().all(|v| ptype.value_is_type(v))
                } else {
                    false
                }
            }
        }
    }
}

impl TryFrom<&str> for ParameterType {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let ptype = match string {
            "string" => Self::String,
            "number" => Self::Number,
            "list" => Self::List,
            "bool" | "boolean" => Self::Boolean,
            other => {
                return Err(
                    ErrorKind::InvalidParameterType(other.to_owned()).into()
                )
            }
        };

        Ok(ptype)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Parameter {
    name: String,
    parameter_types: Vec<ParameterType>,
    optional: bool,
    rest: bool,
}

impl Parameter {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn module(mut self) -> Self {
        self.parameter_types.push(ParameterType::Module);
        self
    }

    pub fn function(mut self) -> Self {
        self.parameter_types.push(ParameterType::Function);
        self
    }

    pub fn list(mut self) -> Self {
        self.parameter_types.push(ParameterType::List);
        self
    }

    pub fn number(mut self) -> Self {
        self.parameter_types.push(ParameterType::Number);
        self
    }

    pub fn string(mut self) -> Self {
        self.parameter_types.push(ParameterType::String);
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn rest(mut self) -> Self {
        self.rest = true;
        self
    }

    pub fn value_is_type(&self, value: &Value) -> Result<()> {
        if self.parameter_types.is_empty()
            || self
                .parameter_types
                .iter()
                .any(|pt| pt.value_is_type(value))
        {
            Ok(())
        } else {
            Err(ErrorKind::InvalidArgument {
                expected: self.parameter_types.clone(),
                actual: value.clone(),
            }
            .into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
}

impl From<Parameter> for Parameters {
    fn from(parameter: Parameter) -> Self {
        Self::new(vec![parameter])
            .expect("Single parameter should always be valid")
    }
}

impl Parameters {
    pub fn new(parameters: Vec<Parameter>) -> Result<Self> {
        if Self::rest_param_is_not_last_param(&parameters) {
            return Err(ErrorKind::NonLastParameterIsRest.into());
        }

        if Self::has_required_param_after_optional(&parameters) {
            return Err(ErrorKind::RequiredParamAfterOptional.into());
        }

        Ok(Self { parameters })
    }

    pub fn none() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }

    pub fn has_rest_param(&self) -> bool {
        // If a `Parameters` object has a rest param, it's always the last one.
        self.parameters.last().map_or(false, |p| p.rest)
    }

    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Parameter> {
        self.parameters.iter()
    }

    pub fn last(&self) -> Option<&Parameter> {
        self.parameters.last()
    }

    fn rest_param_is_not_last_param(parameters: &[Parameter]) -> bool {
        parameters
            .iter()
            .enumerate()
            .any(|(i, param)| param.rest && i != parameters.len() - 1)
    }

    fn has_required_param_after_optional(parameters: &[Parameter]) -> bool {
        // Get first optional parameter
        if let Some(index) = parameters.iter().position(|p| p.optional) {
            // Check that no following params aren't optional
            parameters[index..].iter().any(|p| !p.optional)
        } else {
            false
        }
    }
}

impl TryFrom<&str> for Parameters {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut params = Vec::new();
        let mut optional = false;
        let mut rest = false;

        for param in value.split_whitespace() {
            if param.starts_with("&opt") {
                optional = true;
            } else if param.starts_with('&') {
                rest = true;
            } else if let Some((name, ptype)) = param.split_once(':') {
                params.push(Parameter {
                    name: name.to_owned(),
                    parameter_types: vec![ptype.try_into()?],
                    optional,
                    rest,
                });
            } else {
                params.push(Parameter {
                    name: param.to_owned(),
                    parameter_types: vec![],
                    optional,
                    rest,
                });
            }
        }

        Parameters::new(params)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod test_parameters_from_string {
//         use super::*;

//         #[test]
//         fn test_from_string() {
//             let p: Result<Parameters> = "a b:string &opt c & d".try_into();
//             dbg!(p);
//         }
//     }
// }
