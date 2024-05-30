use std::fmt::Debug;

use statement::StatementType;

pub mod expression;
pub mod statement;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<StatementType>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }

        out
    }
}
