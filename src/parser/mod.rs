pub mod test;

use crate::ast::expression::node::{Identifier, InfixExpression, IntegerLiteral, PrefixExpression};
use crate::ast::expression::ExpressionType;
use crate::ast::statement::node::{ExpressionStatement, LetStatement, ReturnStatement};
use crate::ast::statement::StatementType;
use crate::ast::Program;
use crate::{
    lexer::Lexer,
    token::{Precedence, Token},
};
use core::option::Option;
use std::{collections::HashMap, mem::Discriminant};
use tracing::auto_log;

type PrefixParseFn = fn(&mut Parser) -> Box<ExpressionType>;
type InfixParseFn = fn(&mut Parser, Box<ExpressionType>) -> Box<ExpressionType>;

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,

    prefix_parse_fns: HashMap<Discriminant<Token>, PrefixParseFn>,
    infix_parse_fns: HashMap<Discriminant<Token>, InfixParseFn>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut parser = Parser {
            l,
            errors: Vec::new(),
            cur_token: Token::EOF,
            peek_token: Token::EOF,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.next_token();
        parser.next_token();

        parser.register_prefix(Token::IDENT(String::new()), Parser::parse_identifier);
        parser.register_prefix(Token::INT(0), Parser::parse_integer_literal);
        parser.register_prefix(Token::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(Token::MINUS, Parser::parse_prefix_expression);

        parser.register_infix(Token::PLUS, Parser::parse_infix_expression);
        parser.register_infix(Token::MINUS, Parser::parse_infix_expression);
        parser.register_infix(Token::SLASH, Parser::parse_infix_expression);
        parser.register_infix(Token::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(Token::EQ, Parser::parse_infix_expression);
        parser.register_infix(Token::NOT_EQ, Parser::parse_infix_expression);
        parser.register_infix(Token::LT, Parser::parse_infix_expression);
        parser.register_infix(Token::GT, Parser::parse_infix_expression);

        parser
    }

    #[auto_log]
    pub fn parse_identifier(&mut self) -> Box<ExpressionType> {
        Box::new(ExpressionType::Identifier(Identifier {
            token: self.cur_token.clone(),
        }))
    }

    #[auto_log]
    fn parse_integer_literal(&mut self) -> Box<ExpressionType> {
        Box::new(ExpressionType::IntegerLiteral(IntegerLiteral {
            token: self.cur_token.clone(),
        }))
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        self.peek_token.to_precedence()
    }

    pub fn cur_precedence(&mut self) -> Precedence {
        self.cur_token.to_precedence()
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn cur_token_is(&self, t: Token) -> bool {
        self.cur_token.is(t)
    }

    fn peek_token_is(&self, t: Token) -> bool {
        self.peek_token.is(t)
    }

    fn expect_peek(&mut self, t: &Token) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    #[auto_log]
    fn parse_statement(&mut self) -> Option<Box<StatementType>> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => Some(self.parse_return_statement()),
            _ => Some(self.parse_expression_statement()),
        }
    }

    #[auto_log]
    fn parse_let_statement(&mut self) -> Option<Box<StatementType>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(&Token::IDENT(String::new())) {
            return None;
        }

        let name = self.cur_token.to_string();

        if !self.expect_peek(&Token::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(StatementType::Let(LetStatement {
            token,
            name,
            value: None,
        })))
    }

    #[auto_log]
    fn parse_return_statement(&mut self) -> Box<StatementType> {
        let token = self.cur_token.clone();
        self.next_token();

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Box::new(StatementType::Return(ReturnStatement {
            token,
            value: None,
        }))
    }

    #[auto_log]
    fn parse_expression_statement(&mut self) -> Box<StatementType> {
        let expression = self.parse_expression(Precedence::LOWEST);
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Box::new(StatementType::Expression(ExpressionStatement {
            expression,
        }))
    }

    pub fn no_prefix_parse_fn_error(&mut self, t: Token) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(msg);
    }

    #[auto_log]
    pub fn parse_prefix_expression(&mut self) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);

        Box::new(ExpressionType::Prefix(PrefixExpression {
            operator: token,
            right,
        }))
    }

    #[auto_log]
    pub fn parse_infix_expression(&mut self, left: Box<ExpressionType>) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right: Box<ExpressionType>;
        // if token == Token::PLUS {
        //     println!("token: {:?}", token);
        //     right = self.parse_expression(precedence.sum(-1));
        // } else {
        // }
        right = self.parse_expression(precedence);

        Box::new(ExpressionType::Infix(InfixExpression {
            operator: token,
            left,
            right,
        }))
    }

    #[auto_log]
    fn parse_expression(&mut self, precedence: Precedence) -> Box<ExpressionType> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.cur_token.to_original_type());

        if prefix.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.clone());
            return Box::new(ExpressionType::Identifier(Identifier { token: Token::EOF }));
        }

        let mut left_exp = prefix.unwrap()(self);

        while !self.peek_token_is(Token::SEMICOLON)
            && precedence.to_int() < self.peek_precedence().to_int()
        {
            let infix = self
                .infix_parse_fns
                .get(&self.peek_token.to_original_type())
                .cloned();

            if infix.is_none() {
                return left_exp;
            }

            self.next_token();
            left_exp = infix.unwrap()(self, left_exp);
        }

        left_exp
    }

    #[auto_log]
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }

            self.next_token();
        }
        self.check_parser_errors();

        program
    }

    pub fn register_prefix(&mut self, token_type: Token, function: PrefixParseFn) {
        self.prefix_parse_fns
            .insert(token_type.to_original_type(), function);
    }

    pub fn register_infix(&mut self, token_type: Token, function: InfixParseFn) {
        self.infix_parse_fns
            .insert(token_type.to_original_type(), function);
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, expected: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected, self.peek_token
        );
        self.errors.push(msg);
    }

    fn check_parser_errors(&self) {
        let errors = self.errors();

        if errors.is_empty() {
            return;
        }

        let mut error_msg = format!("\nParser has {} errors\n", errors.len());
        for msg in errors {
            error_msg.push_str(&format!("Parser error: {}\n", msg));
        }

        panic!("{}", error_msg);
    }
}
