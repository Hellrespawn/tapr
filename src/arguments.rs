use crate::error::{Error, ErrorKind};
use crate::parser::parameters::{Parameter, ParameterAmount};
use crate::{Environment, Node, NodeData, ParameterType, Parameters, Result};

pub struct Arguments<'a> {
    parameters: &'a Parameters,
    arguments: Vec<Node>,
}

impl<'a> Arguments<'a> {
    pub fn is_empty(&self) -> bool {
        self.arguments.is_empty()
    }

    pub fn len(&self) -> usize {
        self.arguments.len()
    }

    pub fn arguments(&self) -> &[Node] {
        &self.arguments
    }

    pub fn get(&self, index: usize) -> Option<Node> {
        self.arguments.get(index).cloned()
    }

    pub fn unwrap(&self, index: usize) -> Node {
        self.get(index).expect("Called unwrap on invalid index.")
    }

    pub fn unwrap_from(&self, index: usize) -> Vec<Node> {
        self.arguments[index..].to_vec()
    }

    fn check_amount(&self) -> Result<()> {
        let len = self.len();

        match self.parameters.amount() {
            ParameterAmount::None => {
                if len > 0 {
                    return Err(ErrorKind::Message(format!(
                        "Expected 0 parameters, found {len}."
                    ))
                    .into());
                }
            }
            ParameterAmount::Fixed(amount) => {
                if len != amount {
                    return Err(ErrorKind::Message(format!(
                        "Expected {amount} parameters, found {len}.",
                    ))
                    .into());
                }
            }
            ParameterAmount::Optional(min, max) => {
                if len < min || len > max {
                    return Err(ErrorKind::Message(format!(
                        "Expected {min}-{max} parameters, found {len}.",
                    ))
                    .into());
                }
            }
            ParameterAmount::Rest(min) => {
                if len < min {
                    return Err(ErrorKind::Message(format!(
                        "Expected at least {min} parameters, found {len}.",
                    ))
                    .into());
                }
            }
        }

        Ok(())
    }
}

impl<'a> Arguments<'a> {
    pub fn from_nodes(
        parameters: &'a Parameters,
        arguments: Vec<Node>,
    ) -> Result<Self> {
        let arguments = Self {
            parameters,
            arguments,
        };

        arguments.check_amount()?;
        arguments.check_types()?;

        Ok(arguments)
    }

    pub fn add_to_env(self, env: &mut Environment) -> Result<()> {
        for (parameter, argument) in self.parameters.iter().zip(self.arguments)
        {
            env.def(parameter.get_name().to_owned(), argument)?;
        }

        Ok(())
    }

    pub fn parse_parameters(&self, index: usize) -> Result<Parameters> {
        let node = &self.arguments[index];

        let NodeData::BTuple(nodes) = node.data() else {
            panic!("Called unwrap_keyword on non-Value::Keyword")
        };

        let string = nodes
            .iter()
            .map(|n| {
                if let NodeData::Symbol(symbol) = n.data() {
                    Ok(symbol.clone())
                } else {
                    Err(ErrorKind::InvalidNodeArgument {
                        expected: vec![ParameterType::Symbol],
                        actual: n.clone(),
                    }
                    .into())
                }
            })
            .collect::<Result<Vec<_>>>()?
            .join(" ");

        let parameters = string.as_str().try_into()?;

        Ok(parameters)
    }

    fn check_types(&self) -> Result<()> {
        // Check pairs of params and args, length is already checked here.
        for (param, arg) in self.parameters.iter().zip(&self.arguments) {
            if !param.node_is_type(arg) {
                return Err(Self::create_argument_error(param, arg));
            }
        }

        // If function has a rest parameter...
        if self.parameters.has_rest_param() {
            let remaining_args = self.arguments.get(self.len()..);

            // and there are more args...
            if let Some(remaining_args) = remaining_args {
                let last_param = self.parameters.last().expect("Parameters should have a Parameter if has_rest_param is true.");

                // Check the last param.
                for arg in remaining_args {
                    if !last_param.node_is_type(arg) {
                        return Err(Self::create_argument_error(
                            last_param, arg,
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn create_argument_error(parameter: &Parameter, node: &Node) -> Error {
        ErrorKind::InvalidNodeArgument {
            expected: parameter.types().to_vec(),
            actual: node.clone(),
        }
        .into()
    }
}
