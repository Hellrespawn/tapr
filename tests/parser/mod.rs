use korisp::lexer::Lexer;
use korisp::parser::ast::Node;
use korisp::parser::Parser;

use korisp::Result;

mod test_expression;

pub fn parse(source: &str) -> Result<Node> {
    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);

    let Node::Program(program) = parser.parse()? else {
        panic!("parser.parse() did not return program");
    };

    Ok(*program.expression)
}
