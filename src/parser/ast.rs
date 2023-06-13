use crate::parser::{Parser, Rule};
use crate::visitor::Visitor;
use crate::Result;
use pest::iterators::Pair;
use pest::Parser as PestParser;

#[derive(Debug)]
pub enum Node {
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
}

impl Node {
    pub fn from_string(source: &str) -> Result<Node> {
        let mut pairs = Parser::parse(Rule::main, source)?;

        let node = Node::parse_value(
            pairs.next().expect("Pairs<Rule::main> panicked on next()"),
        );

        Ok(node)
    }

    fn parse_value(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::symbol => Node::parse_symbol(pair),
            Rule::keyword => Node::Keyword(
                pair.into_inner()
                    .next()
                    .expect("Unable to read token from keyword.")
                    .as_str()
                    .to_owned(),
            ),
            Rule::constant => match pair.as_str() {
                "true" => Node::True,
                "false" => Node::False,
                "nil" => Node::Nil,
                other => panic!("Unexpected constant '{other}'"),
            },
            Rule::string => Node::String(
                pair.into_inner()
                    .next()
                    .expect("Rule::string did not have inner text.")
                    .as_str()
                    .to_owned(),
            ),
            Rule::number => {
                Node::Number(pair.as_str().parse().unwrap_or_else(|_| {
                    panic!("unable to parse {} as number", pair.as_str())
                }))
            }
            Rule::special => Node::Special(Box::new(Special::from_pair(pair))),
            Rule::plist => Node::List {
                literal: false,
                nodes: pair.into_inner().map(Node::parse_value).collect(),
            },
            Rule::blist => Node::List {
                literal: true,
                nodes: pair.into_inner().map(Node::parse_value).collect(),
            },
            Rule::main => Node::Main(
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
        }
    }

    fn parse_symbol(pair: Pair<Rule>) -> Node {
        let inner = pair.into_inner().map(|p| p.as_str()).collect::<Vec<_>>();

        match inner.len() {
            1 => Node::Symbol {
                module: None,
                value: inner[0].to_owned(),
            },
            2 => Node::Symbol {
                module: Some(inner[0].to_owned()),
                value: inner[1].to_owned(),
            },
            other => panic!("Rule::symbol has {other} inner pairs."),
        }
    }
}

#[derive(Debug)]
pub enum Special {
    If {
        condition: Node,
        then: Node,
        else_branch: Option<Node>,
    },
    Defn {
        name: String,
        arguments: Vec<Node>,
        body: Vec<Node>,
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
            Rule::if_ => Special::if_(special),
            Rule::defn => Special::defn(special),
            Rule::set => Special::set(special),
            Rule::var => Special::var(special),
            _ => unreachable!(
                "Encountered '{}' inside Rule::special.",
                special.as_str()
            ),
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

    fn defn(pair: Pair<Rule>) -> Special {
        let mut inner = pair.into_inner();

        Special::Defn {
            name: inner
                .next()
                .expect("Rule::defn did not have a name.")
                .as_str()
                .to_owned(),
            arguments: inner
                .next()
                .expect("Rule::defn did not have arguments.")
                .into_inner()
                .map(Node::parse_value)
                .collect(),
            body: inner.map(Node::parse_value).collect(),
        }
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
