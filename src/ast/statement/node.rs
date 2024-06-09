use crate::{
    ast::{expression::ExpressionType, Node},
    token::Token,
};

use super::{Statement, StatementType};

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: String,
    pub value: Box<ExpressionType>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string().to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.to_string());
        out.push_str(" = ");

        out.push_str(&self.value.string());

        out.push_str(";");

        out
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<ExpressionType>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");

        out.push_str(&self.value.string());

        out.push_str(";");

        out
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<ExpressionType>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression.as_ref().token_literal()
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<StatementType>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}
