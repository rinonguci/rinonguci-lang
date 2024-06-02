use crate::{ast::Node, token::Token};

use super::{Expression, ExpressionType};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<ExpressionType>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.operator.to_string());
        out.push_str(&self.right.as_ref().string());
        out.push_str(")");

        out
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct InfixExpression {
    pub operator: Token,
    pub left: Box<ExpressionType>,
    pub right: Box<ExpressionType>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.as_ref().string());
        out.push_str(" ");
        out.push_str(&self.operator.to_string());
        out.push_str(" ");
        out.push_str(&self.right.as_ref().string());
        out.push_str(")");

        out
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}
