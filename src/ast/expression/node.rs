use crate::{
    ast::{statement::StatementType, TNode},
    token::Token,
};

use super::{ExpressionType, TExpression};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl TNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}
impl TExpression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
}

impl TNode for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}
impl TExpression for IntegerLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<ExpressionType>,
}

impl TNode for PrefixExpression {
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

impl TExpression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct InfixExpression {
    pub operator: Token,
    pub left: Box<ExpressionType>,
    pub right: Box<ExpressionType>,
}

impl TNode for InfixExpression {
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

impl TExpression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
}

impl TNode for Boolean {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}

impl TExpression for Boolean {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<ExpressionType>,
    pub consequence: Box<StatementType>,
    pub alternative: Option<Box<StatementType>>,
}

impl TNode for IfExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if ");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str("{ ");
        out.push_str(&self.consequence.string());
        out.push_str(" }");
        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str("{ ");
            out.push_str(&alt.string());
        }
        out
    }
}

impl TExpression for IfExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Box<ExpressionType>>,
    pub body: Box<StatementType>,
}

impl TNode for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = vec![];
        for p in &self.parameters {
            params.push(p.string());
        }
        out.push_str(&self.token_literal());
        out.push_str("(");
        out.push_str(&params.join(","));
        out.push_str(")");
        out.push_str(&self.body.string());
        out
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<ExpressionType>,
    pub arguments: Vec<Box<ExpressionType>>,
}

impl TNode for CallExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut args = vec![];
        for a in &self.arguments {
            args.push(a.string());
        }
        out.push_str(&self.function.string());
        out.push_str("(");
        out.push_str(&args.join(", "));
        out.push_str(")");
        out
    }
}

impl TExpression for CallExpression {
    fn expression_node(&self) {}
}
