use super::Value;
use crate::error::ErrorKind;
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub enum ParameterType {
    Function,
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
            ParameterType::Function => {
                matches!(value, Value::Lambda(_) | Value::Builtin(_))
            }
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
    name: Option<String>,
    parameter_type: ParameterType,
    rest: bool,
}

impl Parameter {
    pub fn new(
        name: String,
        parameter_type: ParameterType,
        rest: bool,
    ) -> Self {
        Self {
            name: Some(name),
            parameter_type,
            rest,
        }
    }

    pub fn anonymous(parameter_type: ParameterType, rest: bool) -> Self {
        Self {
            name: None,
            parameter_type,
            rest,
        }
    }

    pub fn any(name: &str) -> Self {
        Self::new(name.to_owned(), ParameterType::Any, false)
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn value_is_type(&self, value: &Value) -> Result<()> {
        if self.parameter_type.value_is_type(value) {
            Ok(())
        } else {
            Err(ErrorKind::InvalidArgument {
                expected: self.parameter_type,
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
        let has_rest_param_before_last = parameters
            .iter()
            .enumerate()
            .any(|(i, param)| param.rest && i != parameters.len() - 1);

        if has_rest_param_before_last {
            return Err(ErrorKind::NonLastParameterIsRest.into());
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
}
