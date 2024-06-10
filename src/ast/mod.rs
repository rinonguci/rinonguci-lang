use std::{any::Any, fmt::Debug};

use enum_as_inner::EnumAsInner;
use expression::ExpressionType;
use statement::StatementType;

pub mod expression;
pub mod statement;

pub trait TNode: Any {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug, EnumAsInner)]
pub enum Node {
    Statement(StatementType),
    Expression(ExpressionType),
    Program(Program),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<StatementType>>,
}

impl TNode for Program {
    fn token_literal(&self) -> String {
        "Program".into()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }

        out
    }
}

impl Program {
    pub fn to_node(self) -> Node {
        Node::Program(self)
    }
}
