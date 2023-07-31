use self::reader_macro::ReaderMacro;
use crate::env::{DebugAst, DEBUG_AST, DEBUG_PARSER};
use crate::graph::GraphVisitor;
use crate::location::Location;
use crate::{Node, NodeData, Result};
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser as PestParser;

pub mod parameters;
pub mod reader_macro;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

impl Parser {}

pub fn parse_string(source: &str, name: &str) -> Result<Node> {
    let mut pairs = Parser::parse(Rule::main, source)?;

    if *DEBUG_PARSER {
        println!("{pairs:#?}");
    }

    let node =
        parse_pair(pairs.next().expect("Pairs<Rule::main> panicked on next()"));

    if !matches!(*DEBUG_AST, DebugAst::Off) {
        GraphVisitor::create_ast_graph(
            &node,
            name,
            matches!(*DEBUG_AST, DebugAst::RetainDot),
        );
    }

    Ok(node)
}

pub fn parse_pair(pair: Pair<Rule>) -> Node {
    let location = Location::from_pair(&pair);

    let data = match pair.as_rule() {
        Rule::symbol => NodeData::Symbol(
            pair.into_inner()
                .next()
                .expect("Rule::symbol should have Rule::token")
                .as_str()
                .to_owned(),
        ),
        Rule::keyword => {
            NodeData::Keyword(pair.as_str().get(1..).unwrap_or("").to_owned())
        }
        Rule::constant => match pair.as_str() {
            "true" => NodeData::True,
            "false" => NodeData::False,
            "nil" => NodeData::Nil,
            other => unreachable!("Invalid constant '{other}'"),
        },
        Rule::string | Rule::long_string => {
            NodeData::String(extract_string(pair))
        }
        Rule::buffer | Rule::long_buffer => {
            NodeData::Buffer(extract_string(pair))
        }
        Rule::number => {
            NodeData::Number(pair.as_str().parse().unwrap_or_else(|_| {
                panic!("Unable to parse '{}' as number", pair.as_str())
            }))
        }
        Rule::value => return parse_value(pair),
        Rule::p_tuple => {
            NodeData::PTuple(pair.into_inner().map(parse_pair).collect())
        }
        Rule::b_tuple => {
            NodeData::BTuple(pair.into_inner().map(parse_pair).collect())
        }
        Rule::struct_ => NodeData::Struct(
            pair.into_inner().map(parse_pair).tuples().collect(),
        ),
        Rule::p_array => {
            NodeData::PArray(pair.into_inner().map(parse_pair).collect())
        }
        Rule::b_array => {
            NodeData::BArray(pair.into_inner().map(parse_pair).collect())
        }
        Rule::table => NodeData::Table(
            pair.into_inner().map(parse_pair).tuples().collect(),
        ),
        Rule::main => NodeData::Main(
            pair.into_inner()
                .take_while(|p| p.as_rule() != Rule::EOI)
                .map(parse_pair)
                .collect(),
        ),
        rule => {
            unreachable!("Attempted to parse '{:?}':\n'{:#?}'", rule, pair,)
        }
    };

    Node::with_location(location, data)
}

fn extract_string(pair: Pair<Rule>) -> String {
    pair.into_inner()
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_owned()
}

fn parse_value(pair: Pair<Rule>) -> Node {
    let mut pairs = pair.into_inner().collect::<Vec<_>>();

    let pair = pairs
        .pop()
        .expect("Rule::value should always have a raw_value");

    let mut node = parse_pair(pair);

    for pair in pairs.iter().rev() {
        let reader_macro = ReaderMacro::from_pair(pair);
        let location = Location::from_pair(pair);

        let data = NodeData::PTuple(vec![
            Node::with_location(location, NodeData::Symbol(reader_macro.to_string())),
            node,
        ]);

        node = Node::with_location(location, data);
    }

    node
}
