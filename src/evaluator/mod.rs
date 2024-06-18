use anyhow::{anyhow, Result};
use rinonguci_script::new_error;
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expression::{self, node::IfExpression, ExpressionType},
        statement::{
            node::{BlockStatement, ExpressionStatement},
            StatementType,
        },
        Node, Program, TNode,
    },
    object::{
        environment::Environment, Boolean, Function, Integer, Null, Object, ObjectType, ReturnValue,
    },
    token::Token,
};

pub mod test;

pub fn eval(node: Box<Node>, env: Rc<RefCell<Environment>>) -> Object {
    match *node {
        Node::Expression(expr) => eval_expression(expr, env),
        Node::Statement(stmt) => eval_statement(stmt, env),
        Node::Program(program) => eval_program(program, env),
    }
}

fn eval_expression(expr: ExpressionType, env: Rc<RefCell<Environment>>) -> Object {
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
            eval_prefix_expression(operator, eval(right.to_node(), env))
        }
        ExpressionType::Infix(expression::node::InfixExpression {
            left,
            operator,
            right,
        }) => eval_infix_expression(operator, left.to_node(), right.to_node(), env),
        ExpressionType::If(ie) => eval_if_expression(ie, env),
        ExpressionType::Identifier(ident) => {
            let val = env.borrow().get(ident.token.to_string());
            match val {
                Some(val) => val,
                None => new_error!("identifier not found: {}", ident.token.to_string()),
            }
        }
        ExpressionType::Fn(func) => {
            let params = func.parameters;
            let body = func.body;
            Object::Function(Function {
                parameters: params,
                body: body,
                env,
            })
        }
        ExpressionType::Call(call) => {
            let func = eval(call.function.to_node(), Rc::clone(&env));
            if func.is_error() {
                return func;
            }

            let args = eval_expressions(call.arguments, Rc::clone(&env));
            if args.len() == 1 && args[0].is_error() {
                return args[0].clone();
            }

            apply_function(func, args)
        }
    }
}

fn eval_statement(stmt: StatementType, env: Rc<RefCell<Environment>>) -> Object {
    match stmt {
        StatementType::Expression(ExpressionStatement { expression }) => {
            eval(expression.to_node(), env)
        }
        StatementType::Block(BlockStatement { statements }) => eval_statements(statements, env),
        StatementType::Return(node) => {
            let val = eval(node.value.to_node(), env);
            Object::Return(ReturnValue {
                value: Box::new(val),
            })
        }
        StatementType::Let(let_stmt) => {
            if let Some(_) = let_stmt.token {
                let val = eval(let_stmt.value.to_node(), Rc::clone(&env));
                if val.is_error() {
                    return val;
                }
                env.borrow_mut().init(let_stmt.name.to_string(), val)
            } else {
                let is_found = env.borrow().get(let_stmt.name.to_string());
                if is_found.is_none() {
                    return new_error!("identifier not found: {}", let_stmt.name.to_string());
                }

                let val = eval(let_stmt.value.to_node(), Rc::clone(&env));
                if val.is_error() {
                    return val;
                }
                env.borrow_mut().assign(let_stmt.name.to_string(), val)
            }
        }
    }
}

fn eval_program(program: Program, env: Rc<RefCell<Environment>>) -> Object {
    eval_statements(program.statements, env)
}

fn eval_statements(stmts: Vec<Box<StatementType>>, env: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null(Null {});
    for statement in stmts {
        result = eval(statement.to_node(), Rc::clone(&env));

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
        Object::Boolean(Boolean { value }) => Object::Boolean(Boolean { value: !value }),
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

fn eval_infix_expression(
    operator: Token,
    left: Box<Node>,
    right: Box<Node>,
    env: Rc<RefCell<Environment>>,
) -> Object {
    let left = eval(left, Rc::clone(&env));
    let right = eval(right, Rc::clone(&env));

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

fn eval_if_expression(ie: IfExpression, env: Rc<RefCell<Environment>>) -> Object {
    let condition = eval(ie.condition.to_node(), Rc::clone(&env));

    if is_truthy(condition) {
        return eval(ie.consequence.to_node(), Rc::clone(&env));
    } else if ie.alternative.is_some() {
        return eval(ie.alternative.unwrap().to_node(), Rc::clone(&env));
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

fn eval_expressions(exps: Vec<Box<ExpressionType>>, env: Rc<RefCell<Environment>>) -> Vec<Object> {
    let mut result = vec![];
    for e in exps {
        let evaluated = eval(e.to_node(), Rc::clone(&env));
        if evaluated.is_error() {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn apply_function(func_obj: Object, args: Vec<Object>) -> Object {
    if let Object::Function(func_obj) = func_obj {
        let extended_env = extend_function_env(&func_obj, args);
        if extended_env.is_err() {
            return new_error!("{}", extended_env.err().unwrap().to_string());
        }
        let evaluated = eval(func_obj.body.to_node(), extended_env.unwrap());
        unwrap_return_value(evaluated)
    } else {
        new_error!("not a function: {:?}", func_obj.object_type())
    }
}

fn extend_function_env(func_obj: &Function, args: Vec<Object>) -> Result<Rc<RefCell<Environment>>> {
    let env = Environment::new_enclosed_environment(Rc::clone(&func_obj.env));
    if func_obj.parameters.len() != args.len() {
        Err(anyhow!(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            func_obj.parameters.len()
        )))?;
    }
    for (param_idx, param) in func_obj.parameters.iter().enumerate() {
        env.borrow_mut()
            .init(param.as_ref().string(), args[param_idx].clone());
    }
    Ok(env)
}

fn unwrap_return_value(obj: Object) -> Object {
    if let Object::Return(ReturnValue { value }) = obj {
        *value
    } else {
        obj
    }
}
