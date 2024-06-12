use rinonguci_script::new_error;

use crate::{
    ast::{
        expression::{self, node::IfExpression, ExpressionType},
        statement::{
            node::{BlockStatement, ExpressionStatement},
            StatementType,
        },
        Node, Program,
    },
    object::{Boolean, Error, Integer, Null, Object, ObjectType, ReturnValue},
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
        ExpressionType::If(ie) => eval_if_expression(ie),
        _ => Object::Null(Null {}),
    }
}

fn eval_statement(stmt: StatementType) -> Object {
    match stmt {
        StatementType::Expression(ExpressionStatement { expression }) => eval(expression.to_node()),
        StatementType::Block(BlockStatement {
            statements,
            token: _,
        }) => eval_statements(statements),
        StatementType::Return(node) => {
            let val = eval(node.value.to_node());
            Object::Return(ReturnValue {
                value: Box::new(val),
            })
        }
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

        if result.is_return() {
            return result;
        }

        if result.is_error() {
            return result;
        }
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
        _ => new_error!("unknown operator: {:?} {:?}", operator, right.object_type()),
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
        return new_error!("unknown operator: -{:?}", right.object_type());
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
                eval_integer_infix_expression(operator, left, right)
            }
            (left_type, right_type) => {
                if left_type != right_type {
                    return new_error!(
                        "type mismatch: {:?} {} {:?}",
                        left_type,
                        operator.to_string(),
                        right_type
                    );
                } else {
                    return new_error!(
                        "unknown operator: {:?} {} {:?}",
                        left_type,
                        operator.to_string(),
                        right_type
                    );
                }
            }
        },
    }
}

fn eval_integer_infix_expression(operator: Token, left: Object, right: Object) -> Object {
    match (left, right) {
        (
            Object::Integer(Integer { value: left_val }),
            Object::Integer(Integer { value: right_val }),
        ) => match operator {
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
            _ => new_error!("unknown operator: {:?}", operator),
        },
        _ => new_error!("unknown operator: {:?}", operator),
    }
}

fn eval_if_expression(ie: IfExpression) -> Object {
    let condition = eval(ie.condition.to_node());

    if is_truthy(condition) {
        return eval(ie.consequence.to_node());
    } else if ie.alternative.is_some() {
        return eval(ie.alternative.unwrap().to_node());
    } else {
        return Object::Null(Null {});
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null(_) => false,
        Object::Boolean(Boolean { value }) => value,
        _ => true,
    }
}
