use crate::{
    ast::{expression::ExpressionType, TNode},
    token::Token,
};

use super::StatementType;

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub token: Option<Token>,
    pub name: String,
    pub value: Box<ExpressionType>,
}

impl TNode for LetStatement {
    fn token_literal(&self) -> String {
        match &self.token {
            Some(t) => format!("{} ", t.to_string()),
            None => "".into(),
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(&self.name.to_string());
        out.push_str(" = ");

        out.push_str(&self.value.string());

        out.push_str(";");

        out
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Box<ExpressionType>,
}

impl TNode for ReturnStatement {
    fn token_literal(&self) -> String {
        "return".into()
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

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: Box<ExpressionType>,
}

impl TNode for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.expression.as_ref().token_literal()
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Box<StatementType>>,
}

impl TNode for BlockStatement {
    fn token_literal(&self) -> String {
        "".into()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str("{");
            out.push_str(&stmt.string());
            out.push_str("}");
        }
        out
    }
}
