use crate::error::{Error, ErrorKind};
use crate::Result;

mod parameter;
mod parameter_type;

pub use parameter::Parameter;
pub use parameter_type::ParameterType;

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
        self.parameters
            .last()
            .map_or(false, Parameter::is_rest_param)
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
        parameters.iter().enumerate().any(|(i, param)| {
            param.is_rest_param() && i != parameters.len() - 1
        })
    }

    fn has_required_param_after_optional(parameters: &[Parameter]) -> bool {
        // Get first optional parameter
        if let Some(index) = parameters.iter().position(Parameter::is_optional)
        {
            // Check that no following params aren't optional
            parameters[index..].iter().any(|p| !p.is_optional())
        } else {
            false
        }
    }
}

impl TryFrom<&str> for Parameters {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let value = if value.starts_with('[') && value.ends_with(']') {
            &value[1..value.len() - 2]
        } else {
            value
        };

        let mut optional = false;
        let mut rest = false;

        let params = value
            .split_whitespace()
            .filter_map(|s| match s {
                "&opt" => {
                    optional = true;
                    None
                }
                "&" => {
                    rest = true;
                    None
                }
                symbol => Some(symbol.try_into().map(|p: Parameter| {
                    let p = if optional { p.optional() } else { p };

                    if rest {
                        p.rest()
                    } else {
                        p
                    }
                })),
            })
            .collect::<Result<Vec<Parameter>>>()?;

        Parameters::new(params)
    }
}

impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args = self
            .parameters
            .iter()
            .map(|p| {
                if p.types().is_empty() {
                    p.get_name().to_owned()
                } else {
                    format!(
                        "{}:{}",
                        p.name(),
                        p.types()
                            .iter()
                            .map(std::string::ToString::to_string)
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
            })
            .collect::<Vec<_>>();

        if let Some(first_optional_index) =
            self.parameters.iter().position(Parameter::is_optional)
        {
            args.insert(first_optional_index, "&opt".to_owned());
        }

        if self
            .parameters
            .iter()
            .last()
            .map_or(false, Parameter::is_rest_param)
        {
            args.insert(args.len() - 1, "&".to_owned());
        }

        write!(f, "[{}]", args.join(" "))
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
