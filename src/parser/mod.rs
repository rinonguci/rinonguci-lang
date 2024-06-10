use crate::ast::expression::node::{
    Boolean, CallExpression, FunctionLiteral, Identifier, IfExpression, InfixExpression,
    IntegerLiteral, PrefixExpression,
};
use crate::ast::expression::ExpressionType;
use crate::ast::statement::node::{
    BlockStatement, ExpressionStatement, LetStatement, ReturnStatement,
};
use crate::ast::statement::StatementType;
use crate::ast::Program;
use crate::{
    lexer::Lexer,
    token::{Precedence, Token},
};
use anyhow::{anyhow, Result};
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

        parser.register_prefix(Token::IDENT(String::new()), Parser::parse_identifier);
        parser.register_prefix(Token::INT(0), Parser::parse_integer_literal);
        parser.register_prefix(Token::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(Token::MINUS, Parser::parse_prefix_expression);
        parser.register_prefix(Token::TRUE, Parser::parse_boolean);
        parser.register_prefix(Token::FALSE, Parser::parse_boolean);
        parser.register_prefix(Token::LPAREN, Parser::parse_grouped_expression);
        parser.register_prefix(Token::IF, Parser::parse_if_expression);
        parser.register_prefix(Token::FUNCTION, Parser::parse_function_literal);

        parser.register_infix(Token::PLUS, Parser::parse_infix_expression);
        parser.register_infix(Token::MINUS, Parser::parse_infix_expression);
        parser.register_infix(Token::SLASH, Parser::parse_infix_expression);
        parser.register_infix(Token::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(Token::EQ, Parser::parse_infix_expression);
        parser.register_infix(Token::NOT_EQ, Parser::parse_infix_expression);
        parser.register_infix(Token::LT, Parser::parse_infix_expression);
        parser.register_infix(Token::GT, Parser::parse_infix_expression);
        parser.register_infix(Token::LPAREN, Parser::parse_call_expression);

        parser.next_token();
        parser.next_token();

        parser
    }

    #[auto_log]
    fn parse_identifier(&mut self) -> Box<ExpressionType> {
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

    #[auto_log]
    fn parse_boolean(&mut self) -> Box<ExpressionType> {
        Box::new(ExpressionType::Boolean(Boolean {
            token: self.cur_token.clone(),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Box<ExpressionType> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(&Token::RPAREN) {
            return Box::new(ExpressionType::Identifier(Identifier { token: Token::EOF }));
        }
        exp
    }

    fn parse_block_statement(&mut self) -> Box<StatementType> {
        let token = self.cur_token.clone();
        self.next_token();

        let mut statements = Vec::new();
        while !self.cur_token_is(Token::RBRACE) && !self.cur_token_is(Token::EOF) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(_e) => {}
            }
            self.next_token();
        }

        Box::new(StatementType::Block(BlockStatement { token, statements }))
    }

    fn parse_if_expression(&mut self) -> Box<ExpressionType> {
        let default = Box::new(ExpressionType::Identifier(Identifier { token: Token::EOF }));

        let token = self.cur_token.clone();
        if !self.expect_peek(&Token::LPAREN) {
            return default;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(&Token::RPAREN) {
            return default;
        }

        if !self.expect_peek(&Token::LBRACE) {
            return default;
        }

        let consequence = self.parse_block_statement();

        let mut alternative: Option<Box<StatementType>> = None;

        if self.peek_token_is(Token::ELSE) {
            self.next_token();
            if !self.expect_peek(&Token::LBRACE) {
                return default;
            }
            alternative = Some(self.parse_block_statement());
        }

        Box::new(ExpressionType::If(IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }))
    }

    fn parse_function_parameters(&mut self) -> Vec<Box<ExpressionType>> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(Token::RPAREN) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        identifiers.push(Box::new(ExpressionType::Identifier(Identifier {
            token: self.cur_token.clone(),
        })));
        while self.peek_token_is(Token::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(Box::new(ExpressionType::Identifier(Identifier {
                token: self.cur_token.clone(),
            })));
        }
        if !self.expect_peek(&Token::RPAREN) {
            return Vec::new();
        }
        identifiers
    }

    fn parse_function_literal(&mut self) -> Box<ExpressionType> {
        let default = Box::new(ExpressionType::Identifier(Identifier { token: Token::EOF }));

        let token = self.cur_token.clone();
        if !self.expect_peek(&Token::LPAREN) {
            return default;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(&Token::LBRACE) {
            return default;
        }

        let body = self.parse_block_statement();

        Box::new(ExpressionType::Fn(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_call_expression(&mut self, function: Box<ExpressionType>) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments();

        Box::new(ExpressionType::Call(CallExpression {
            token,
            function,
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<ExpressionType>> {
        let mut arguments = Vec::new();
        if self.peek_token_is(Token::RPAREN) {
            self.next_token();
            return arguments;
        }
        self.next_token();
        arguments.push(self.parse_expression(Precedence::LOWEST));
        while self.peek_token_is(Token::COMMA) {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(Precedence::LOWEST));
        }
        if !self.expect_peek(&Token::RPAREN) {
            return Vec::new();
        }
        arguments
    }

    fn peek_precedence(&mut self) -> Precedence {
        self.peek_token.to_precedence()
    }

    fn cur_precedence(&mut self) -> Precedence {
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
    fn parse_statement(&mut self) -> Result<Box<StatementType>> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    #[auto_log]
    fn parse_let_statement(&mut self) -> Result<Box<StatementType>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(&Token::IDENT(String::new())) {
            return Err(anyhow!("failed to parse let statement"));
        }

        let name = self.cur_token.to_string();

        if !self.expect_peek(&Token::ASSIGN) {
            return Err(anyhow!("failed to parse let statement"));
        }
        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(StatementType::Let(LetStatement {
            token,
            name,
            value,
        })))
    }

    #[auto_log]
    fn parse_return_statement(&mut self) -> Result<Box<StatementType>> {
        let token = self.cur_token.clone();
        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(StatementType::Return(ReturnStatement {
            token,
            value,
        })))
    }

    #[auto_log]
    fn parse_expression_statement(&mut self) -> Result<Box<StatementType>> {
        let expression = self.parse_expression(Precedence::LOWEST);
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(StatementType::Expression(ExpressionStatement {
            expression,
        })))
    }

    fn no_prefix_parse_fn_error(&mut self, t: Token) {
        let msg = format!("no prefix parse function for {:?} found", t.to_string());
        self.errors.push(msg);
    }

    #[auto_log]
    fn parse_prefix_expression(&mut self) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);

        Box::new(ExpressionType::Prefix(PrefixExpression {
            operator: token,
            right,
        }))
    }

    #[auto_log]
    fn parse_infix_expression(&mut self, left: Box<ExpressionType>) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);

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
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(_e) => {}
            }

            self.next_token();
        }
        self.check_parser_errors();

        program
    }

    fn register_prefix(&mut self, token_type: Token, function: PrefixParseFn) {
        self.prefix_parse_fns
            .insert(token_type.to_original_type(), function);
    }

    fn register_infix(&mut self, token_type: Token, function: InfixParseFn) {
        self.infix_parse_fns
            .insert(token_type.to_original_type(), function);
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, expected: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected.to_string(),
            self.peek_token.to_string()
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
