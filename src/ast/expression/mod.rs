pub mod node;

use std::fmt::Debug;

use enum_as_inner::EnumAsInner;
use node::{Identifier, InfixExpression, IntegerLiteral, PrefixExpression};

use crate::ast::Node;

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[derive(Debug, EnumAsInner)]
pub enum ExpressionType {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

impl Node for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.token_literal(),
            ExpressionType::IntegerLiteral(expr) => expr.token_literal(),
            ExpressionType::Prefix(expr) => expr.token_literal(),
            ExpressionType::Infix(expr) => expr.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.string(),
            ExpressionType::IntegerLiteral(expr) => expr.string(),
            ExpressionType::Prefix(expr) => expr.string(),
            ExpressionType::Infix(expr) => expr.string(),
        }
    }
}
