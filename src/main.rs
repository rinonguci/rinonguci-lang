use crate::parser::Parser;

use ast::TNode;
use evaluator::eval;
use lexer::Lexer;

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;

fn main() {
    let input = r#"
      (5 + 10 * 2 + 15 / 3) * 2 + -10;
      (5 + 10 * 2 + 15 / 3) * 2 + -11;
        "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    println!("EQ === {:#?}", program);
    println!("EQ === {:#?}", program.string());

    println!("EQ === {:#?}", eval(Box::new(program.to_node())));
}
