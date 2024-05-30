use crate::{
    ast::{expression::ExpressionType, Node},
    token::Token,
};

use super::Statement;

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<String>,
    pub value: Option<Box<ExpressionType>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string().to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.as_ref().unwrap().to_string());
        out.push_str(" = ");

        if let Some(ref value) = self.value {
            out.push_str(&value.string());
        }

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
    pub value: Option<Box<ExpressionType>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");

        if let Some(ref value) = self.value {
            out.push_str(&value.string());
        }

        out.push_str(";");

        out
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<ExpressionType>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        if let Some(ref expression) = self.expression {
            expression.string()
        } else {
            String::new()
        }
    }
}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}
