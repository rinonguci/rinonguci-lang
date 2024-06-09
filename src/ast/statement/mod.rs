pub mod node;

use enum_as_inner::EnumAsInner;
use node::{BlockStatement, ExpressionStatement, LetStatement, ReturnStatement};
use std::fmt::Debug;

use crate::ast::Node;

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

#[derive(Debug, EnumAsInner)]
pub enum StatementType {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::Let(stmt) => stmt.token_literal(),
            StatementType::Return(stmt) => stmt.token_literal(),
            StatementType::Expression(stmt) => stmt.token_literal(),
            StatementType::Block(stmt) => stmt.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementType::Let(stmt) => stmt.string(),
            StatementType::Return(stmt) => stmt.string(),
            StatementType::Expression(stmt) => stmt.string(),
            StatementType::Block(stmt) => stmt.string(),
        }
    }
}
