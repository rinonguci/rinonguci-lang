use crate::{ast::Node, parser::Parser};
use lexer::Lexer;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() {
    let input = r#"
          let five = 5;
          let ten = 10;

          return 10;
          return 4;
        "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    println!("len: {:#?}", program.string());
    for statement in program.statements {
        println!("{:?}", statement.token_literal());
    }
}
