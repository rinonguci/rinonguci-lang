use crate::{ ast::Node, parser::Parser };
use lexer::Lexer;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() {
    let input = r#"
          5 + 5;
        "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    println!("code: {:#?}", program.string());
}
