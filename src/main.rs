use repl::run_repl;

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    run_repl();
}

// use std::rc::Rc;

// use crate::parser::Parser;

// use ast::TNode;
// use evaluator::eval;
// use lexer::Lexer;
// use object::environment::Environment;

// pub mod ast;
// pub mod evaluator;
// pub mod lexer;
// pub mod object;
// pub mod parser;
// pub mod token;

// fn main() {
//     let input = r#"
//     x=10;
//     x
//         "#;

//     let l = Lexer::new(input.to_string());
//     let mut p = Parser::new(l);
//     let program = p.parse_program();
//     let env = Environment::new();

//     println!("EQ === {:#?}", program);
//     println!("EQ === {:#?}", program.string());

//     println!(
//         "EQ === {:#?}",
//         eval(Box::new(program.to_node()), Rc::clone(&env))
//     );
// }
