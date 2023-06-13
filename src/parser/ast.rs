use crate::location::Location;
use crate::parser::{Parser, Rule};
use crate::visitor::Visitor;
use crate::Result;
use pest::iterators::Pair;
use pest::Parser as PestParser;

use super::DEBUG_PARSER;

#[derive(Debug, Clone)]
pub struct Node {
    location: Location,
    data: NodeData,
}

#[derive(Debug, Clone)]
pub enum NodeData {
    Main(Vec<Node>),
    Special(Box<Special>),
    List {
        literal: bool,
        nodes: Vec<Node>,
    },
    Symbol {
        module: Option<String>,
        value: String,
    },
    Keyword(String),
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl Node {
    pub fn accept<T: std::fmt::Debug>(
        &self,
        visitor: &mut dyn Visitor<T>,
    ) -> T {
        visitor.visit_node(self)
    }

    pub fn data(&self) -> &NodeData {
        &self.data
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn from_string(source: &str) -> Result<Node> {
        let mut pairs = Parser::parse(Rule::main, source)?;

        if *DEBUG_PARSER {
            println!("{pairs:#?}");
        }

        let node = Node::parse_value(
            pairs.next().expect("Pairs<Rule::main> panicked on next()"),
        );

        Ok(node)
    }

    fn parse_value(pair: Pair<Rule>) -> Node {
        let location = Location::from_pair(&pair);

        let data = match pair.as_rule() {
            Rule::symbol => return Node::parse_symbol(pair),
            Rule::keyword => NodeData::Keyword(
                pair.into_inner()
                    .next()
                    .expect("Unable to read token from keyword.")
                    .as_str()
                    .to_owned(),
            ),

            Rule::constant => match pair.as_str() {
                "true" => NodeData::True,
                "false" => NodeData::False,
                "nil" => NodeData::Nil,
                other => panic!("Unexpected constant '{other}'"),
            },
            Rule::string => NodeData::String(
                pair.into_inner()
                    .next()
                    .expect("Rule::string did not have inner text.")
                    .as_str()
                    .to_owned(),
            ),
            Rule::number => {
                NodeData::Number(pair.as_str().parse().unwrap_or_else(|_| {
                    panic!("unable to parse {} as number", pair.as_str())
                }))
            }
            Rule::special => {
                NodeData::Special(Box::new(Special::from_pair(pair)))
            }
            Rule::plist => NodeData::List {
                literal: false,
                nodes: pair.into_inner().map(Node::parse_value).collect(),
            },
            Rule::blist => NodeData::List {
                literal: true,
                nodes: pair.into_inner().map(Node::parse_value).collect(),
            },
            Rule::main => NodeData::Main(
                pair.into_inner()
                    .take_while(|p| p.as_rule() != Rule::EOI)
                    .map(Node::parse_value)
                    .collect(),
            ),
            rule => unreachable!(
                "Attempted to parse '{:?}':\n'{}'",
                rule,
                pair.as_str(),
            ),
        };

        Node { location, data }
    }

    fn parse_symbol(pair: Pair<Rule>) -> Node {
        let location = Location::from_pair(&pair);

        let inner = pair.into_inner().map(|p| p.as_str()).collect::<Vec<_>>();

        let data = match inner.len() {
            1 => NodeData::Symbol {
                module: None,
                value: inner[0].to_owned(),
            },
            2 => NodeData::Symbol {
                module: Some(inner[0].to_owned()),
                value: inner[1].to_owned(),
            },
            other => panic!("Rule::symbol has {other} inner pairs."),
        };

        Node { location, data }
    }
}

#[derive(Debug, Clone)]
pub enum Special {
    Fn {
        parameters: Vec<String>,
        body: Vec<Node>,
    },
    If {
        condition: Node,
        then: Node,
        else_branch: Option<Node>,
    },
    Import {
        name: String,
        prefix: Option<String>,
    },
    Set {
        name: String,
        value: Node,
    },
    Var {
        name: String,
        value: Node,
    },
}

impl Special {
    fn from_pair(pair: Pair<Rule>) -> Special {
        let special = pair
            .into_inner()
            .next()
            .expect("Rule::special did not have inner pair.");

        match special.as_rule() {
            Rule::defn => Special::defn(special),
            Rule::fn_ => Special::fn_(special),
            Rule::if_ => Special::if_(special),
            Rule::import => Special::import(special),
            Rule::set => Special::set(special),
            Rule::var => Special::var(special),
            _ => unreachable!(
                "Encountered '{}' inside Rule::special.",
                special.as_str()
            ),
        }
    }

    fn defn(pair: Pair<Rule>) -> Special {
        let location = Location::from_pair(&pair);

        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .expect("Rule::defn did not have a name.")
            .as_str()
            .to_owned();

        let fn_data = NodeData::Special(Box::new(Special::Fn {
            parameters: inner
                .next()
                .expect("Rule::defn did not have arguments.")
                .into_inner()
                .map(|p| p.as_str().to_owned())
                .collect(),
            body: inner.map(Node::parse_value).collect(),
        }));

        let fn_node = Node {
            location,
            data: fn_data,
        };

        Special::Var {
            name,
            value: fn_node,
        }
    }

    fn fn_(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        Special::Fn {
            parameters: inner
                .next()
                .expect("Rule::defn did not have arguments.")
                .into_inner()
                .map(|p| p.as_str().to_owned())
                .collect(),
            body: inner.map(Node::parse_value).collect(),
        }
    }

    fn if_(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        Special::If {
            condition: Node::parse_value(
                inner.next().expect("Rule::if_ should have condition"),
            ),
            then: Node::parse_value(
                inner.next().expect("Rule::if_ should have then-branch"),
            ),
            else_branch: inner.next().map(Node::parse_value),
        }
    }

    fn import(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .expect("Rule::import did not have name.")
            .as_str()
            .to_owned();

        let prefix = inner.next().map(|p| p.as_str().to_owned());

        Special::Import { name, prefix }
    }

    fn set(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        Special::Set {
            name: inner
                .next()
                .expect("Rule::set did not have name")
                .as_str()
                .to_owned(),
            value: Node::parse_value(
                inner.next().expect("Rule::set did not have value"),
            ),
        }
    }

    fn var(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        Special::Var {
            name: inner
                .next()
                .expect("Rule::var did not have name")
                .as_str()
                .to_owned(),
            value: Node::parse_value(
                inner.next().expect("Rule::var did not have value"),
            ),
        }
    }
}
