use crate::{
    ast::{
        expression::{self, ExpressionType},
        statement::{node::ExpressionStatement, StatementType},
        Node, Program,
    },
    object::{Boolean, Integer, Null, Object, ObjectType},
    token::Token,
};

pub mod test;

pub fn eval(node: Box<Node>) -> Object {
    match *node {
        Node::Expression(expr) => eval_expression(expr),
        Node::Statement(stmt) => eval_statement(stmt),
        Node::Program(program) => eval_program(program),
    }
}

fn eval_expression(expr: ExpressionType) -> Object {
    match expr {
        ExpressionType::IntegerLiteral(expression::node::IntegerLiteral { token }) => {
            Object::Integer(Integer {
                value: token.as_int().unwrap().clone(),
            })
        }
        ExpressionType::Boolean(expression::node::Boolean { token }) => {
            evel_boolean_expression(token)
        }
        ExpressionType::Prefix(expression::node::PrefixExpression { operator, right }) => {
            eval_prefix_expression(operator, eval(right.to_node()))
        }
        ExpressionType::Infix(expression::node::InfixExpression {
            left,
            operator,
            right,
        }) => eval_infix_expression(operator, left.to_node(), right.to_node()),
        _ => Object::Null(Null {}),
    }
}

fn eval_statement(stmt: StatementType) -> Object {
    match stmt {
        StatementType::Expression(ExpressionStatement { expression }) => eval(expression.to_node()),
        _ => Object::Null(Null {}),
    }
}

fn eval_program(program: Program) -> Object {
    eval_statements(program.statements)
}

fn eval_statements(stmts: Vec<Box<StatementType>>) -> Object {
    let mut result = Object::Null(Null {});
    for statement in stmts {
        result = eval(statement.to_node());
    }

    result
}

fn evel_boolean_expression(token: Token) -> Object {
    match token {
        Token::TRUE => Object::Boolean(Boolean { value: true }),
        Token::FALSE => Object::Boolean(Boolean { value: false }),
        _ => Object::Null(Null {}),
    }
}

fn eval_prefix_expression(operator: Token, right: Object) -> Object {
    match operator {
        Token::BANG => eval_bang_operator_expression(right),
        Token::MINUS => evel_minus_prefix_operator_expression(right),
        _ => Object::Null(Null {}),
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(Boolean { value: true }) => Object::Boolean(Boolean { value: false }),
        Object::Boolean(Boolean { value: false }) => Object::Boolean(Boolean { value: true }),
        Object::Null(Null {}) => Object::Boolean(Boolean { value: true }),
        _ => Object::Boolean(Boolean { value: false }),
    }
}

fn evel_minus_prefix_operator_expression(right: Object) -> Object {
    if right.object_type() != ObjectType::INTEGER {
        return Object::Null(Null {});
    }
    let value = right.as_integer().unwrap().value;
    Object::Integer(Integer { value: -value })
}

fn eval_infix_expression(operator: Token, left: Box<Node>, right: Box<Node>) -> Object {
    let left = eval(left);
    let right = eval(right);

    match operator {
        Token::EQ => Object::Boolean(Boolean {
            value: left == right,
        }),
        Token::NOT_EQ => Object::Boolean(Boolean {
            value: left != right,
        }),
        _ => match (left.object_type(), right.object_type()) {
            (ObjectType::INTEGER, ObjectType::INTEGER) => {
                let left = left.as_integer().unwrap();
                let right = right.as_integer().unwrap();
                eval_integer_infix_expression(operator, left, right)
            }
            _ => Object::Null(Null {}),
        },
    }
}

fn eval_integer_infix_expression(operator: Token, left: &Integer, right: &Integer) -> Object {
    let left_val = left.value;
    let right_val = right.value;

    match operator {
        Token::PLUS => Object::Integer(Integer {
            value: left_val + right_val,
        }),
        Token::MINUS => Object::Integer(Integer {
            value: left_val - right_val,
        }),
        Token::ASTERISK => Object::Integer(Integer {
            value: left_val * right_val,
        }),
        Token::SLASH => Object::Integer(Integer {
            value: left_val / right_val,
        }),
        Token::LT => Object::Boolean(Boolean {
            value: left_val < right_val,
        }),
        Token::GT => Object::Boolean(Boolean {
            value: left_val > right_val,
        }),
        Token::EQ => Object::Boolean(Boolean {
            value: left_val == right_val,
        }),
        Token::NOT_EQ => Object::Boolean(Boolean {
            value: left_val != right_val,
        }),
        _ => Object::Null(Null {}),
    }
}
