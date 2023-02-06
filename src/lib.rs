pub mod lexer;
pub mod token;

pub fn main() {
    let source = "(a 1.2 \"string\")";

    let lexer = lexer::Lexer::new(source);

    let tokens = lexer.collect::<Vec<_>>();

    println!("{tokens:?}");
}
