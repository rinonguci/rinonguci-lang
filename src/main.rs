use crate::parser::Parser;

use ast::Node;
use lexer::Lexer;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() {
    let input = r#"
      a + add(b * c) + d
        "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    println!("EQ === {:#?}", program);
    println!("EQ === {:#?}", program.string());
}
