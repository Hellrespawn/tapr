use korisp::lexer::Lexer;
use korisp::parser::ast::Expression;
use korisp::parser::Parser;

use korisp::Result;

mod test_expression;

pub fn parse(source: &str) -> Result<Expression> {
    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);

    let expression = parser.parse()?;

    Ok(expression)
}
