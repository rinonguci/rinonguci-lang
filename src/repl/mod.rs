use std::io::{self, Write};

use crate::{evaluator::eval, lexer::Lexer, object::environment::Environment, parser::Parser};

pub fn run_repl() {
    println!("Welcome to the REPL CLI. Type 'exit' to quit.");
    let env = &mut Environment::new();
    loop {
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "exit\n" {
            break;
        }

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program();

        let x = eval(Box::new(program.to_node()), env);
        println!("{:?}", x);
    }

    println!("Exit REPL!");
}
