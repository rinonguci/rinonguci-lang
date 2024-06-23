pub mod node;

use std::fmt::Debug;

use enum_as_inner::EnumAsInner;
use node::{
    Boolean, CallExpression, FunctionLiteral, Identifier, IfExpression, InfixExpression,
    IntegerLiteral, PrefixExpression, StringLiteral,
};

use crate::ast::TNode;

use super::Node;

#[derive(Debug, EnumAsInner, PartialEq, Clone)]
pub enum ExpressionType {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Boolean(Boolean),
    If(IfExpression),
    Fn(FunctionLiteral),
    Call(CallExpression),
}

impl TNode for ExpressionType {
    fn token_literal(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.token_literal(),
            ExpressionType::IntegerLiteral(expr) => expr.token_literal(),
            ExpressionType::StringLiteral(expr) => expr.token_literal(),
            ExpressionType::Prefix(expr) => expr.token_literal(),
            ExpressionType::Infix(expr) => expr.token_literal(),
            ExpressionType::Boolean(expr) => expr.token_literal(),
            ExpressionType::If(expr) => expr.token_literal(),
            ExpressionType::Fn(expr) => expr.token_literal(),
            ExpressionType::Call(expr) => expr.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionType::Identifier(expr) => expr.string(),
            ExpressionType::IntegerLiteral(expr) => expr.string(),
            ExpressionType::StringLiteral(expr) => expr.string(),
            ExpressionType::Prefix(expr) => expr.string(),
            ExpressionType::Infix(expr) => expr.string(),
            ExpressionType::Boolean(expr) => expr.string(),
            ExpressionType::If(expr) => expr.string(),
            ExpressionType::Fn(expr) => expr.string(),
            ExpressionType::Call(expr) => expr.string(),
        }
    }
}

impl ExpressionType {
    pub fn to_node(self: Box<ExpressionType>) -> Box<Node> {
        Box::new(Node::Expression(*self))
    }
}
