use super::parameter_type::ParameterType;
use crate::interpreter::Value;

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
        Self { name, parameter_types, optional, rest }
    }

    pub fn empty(name: String) -> Self {
        Self { name, ..Default::default() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn module(mut self) -> Self {
        self.parameter_types.push(ParameterType::Module);
        self
    }

    #[must_use]
    pub fn function(mut self) -> Self {
        self.parameter_types.push(ParameterType::Function);
        self
    }

    #[must_use]
    pub fn list(mut self) -> Self {
        self.parameter_types.push(ParameterType::List);
        self
    }

    #[must_use]
    pub fn keyword(mut self) -> Self {
        self.parameter_types.push(ParameterType::Keyword);
        self
    }

    #[must_use]
    pub fn number(mut self) -> Self {
        self.parameter_types.push(ParameterType::Number);
        self
    }

    #[must_use]
    pub fn string(mut self) -> Self {
        self.parameter_types.push(ParameterType::String);
        self
    }

    #[must_use]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    #[must_use]
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
        self.optional
    }

    pub fn types(&self) -> &[ParameterType] {
        &self.parameter_types
    }

    pub fn value_is_type(&self, value: &Value) -> bool {
        self.parameter_types.is_empty()
            || self.parameter_types.iter().any(|pt| pt.value_is_type(value))
    }
}
