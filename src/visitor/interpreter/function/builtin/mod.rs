mod arithmetic;
mod boolean;

pub use arithmetic::*;
pub use boolean::*;

use super::{Arguments, Function};
use crate::lexer::Lexer;
use crate::parser::ast::{Atom, Node};
use crate::parser::Parser;
use crate::visitor::interpreter::{Interpreter, Value};
use crate::{Error, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Write;

struct PrintFunction;

impl PrintFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for PrintFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args =
            PrintFunction::ARGUMENTS.evaluate(intp, arguments_nodes)?;

        println!("{}", &evaluated_args[0]);

        Ok(Value::Nil)
    }
}

struct ReadFunction;

impl ReadFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for ReadFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_arg = ReadFunction::ARGUMENTS
            .evaluate(intp, argument_nodes)?
            .pop()
            .unwrap();

        if let Value::String(prompt) = evaluated_arg {
            print!("{prompt}");
            std::io::stdout().flush()?;

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;

            Ok(Value::String(buffer.trim_end().to_owned()))
        } else {
            Err(Error::InvalidArguments {
                expected: "String",
                values: vec![evaluated_arg],
            })
        }
    }
}

struct EvalFunction;

impl EvalFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for EvalFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args =
            EvalFunction::ARGUMENTS.evaluate(intp, argument_nodes)?;

        let value = &evaluated_args[0];

        if let Value::String(source) = value {
            let lexer = Lexer::new(source);
            let mut parser = Parser::with_number(lexer, intp.parser_no);

            intp.parser_no += 1;

            let program = parser.parse()?;

            let mut intp = Interpreter::new();
            let value = intp.interpret(&program)?;

            Ok(value)
        } else {
            Err(Error::InvalidArguments {
                expected: "String",
                values: evaluated_args,
            })
        }
    }
}
struct QuoteFunction;

impl QuoteFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for QuoteFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        QuoteFunction::ARGUMENTS.check_amount(argument_nodes.len())?;

        let argument = &argument_nodes[0];

        match argument {
            Node::Atom(Atom::Symbol(symbol)) => {
                Ok(Value::Symbol(symbol.lexeme().to_owned()))
            }
            Node::List(list) => {
                let values = list
                    .expressions
                    .iter()
                    .map(|node| node.accept(intp))
                    .collect::<Result<Vec<_>>>()?;

                Ok(Value::List(values))
            }
            _ => {
                let mut values =
                    QuoteFunction::ARGUMENTS.evaluate(intp, argument_nodes)?;

                Ok(values.pop().unwrap())
            }
        }
    }
}

pub static BUILTIN_FUNCTIONS: Lazy<HashMap<&str, Box<dyn Function>>> =
    Lazy::new(|| {
        let mut map: HashMap<&str, Box<dyn Function>> = HashMap::new();

        map.insert(
            "+",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Add, 2)),
        );

        map.insert(
            "-",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Subtract, 2)),
        );

        map.insert(
            "*",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Multiply, 2)),
        );

        map.insert(
            "/",
            Box::new(ArithmeticFunction::new(ArithmeticOp::Divide, 2)),
        );

        map.insert(">", Box::new(BooleanFunction::new(BooleanOp::Greater, 2)));
        map.insert(
            ">=",
            Box::new(BooleanFunction::new(BooleanOp::GreaterOrEqual, 2)),
        );
        map.insert("==", Box::new(BooleanFunction::new(BooleanOp::Equal, 2)));
        map.insert(
            "<=",
            Box::new(BooleanFunction::new(BooleanOp::LessOrEqual, 2)),
        );
        map.insert("<", Box::new(BooleanFunction::new(BooleanOp::Less, 2)));

        map.insert("quote", Box::new(QuoteFunction));
        map.insert("print", Box::new(PrintFunction));
        map.insert("read", Box::new(ReadFunction));
        map.insert("eval", Box::new(EvalFunction));
        map.insert("inc", Box::new(Increment));

        map
    });
