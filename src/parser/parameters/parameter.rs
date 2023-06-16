use super::parameter_type::ParameterType;
use crate::error::ErrorKind;
use crate::interpreter::Value;
use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct Parameter {
    name: String,
    parameter_types: Vec<ParameterType>,
    optional: bool,
    rest: bool,
}

impl Parameter {
    pub fn new(
        name: String,
        parameter_types: Vec<ParameterType>,
        optional: bool,
        rest: bool,
    ) -> Self {
        Self {
            name,
            parameter_types,
            optional,
            rest,
        }
    }

    pub fn empty(name: String) -> Self {
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_rest_param(&self) -> bool {
        self.rest
    }

    pub fn is_optional(&self) -> bool {
        self.rest
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
