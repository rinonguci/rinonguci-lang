pub mod test;

use tracing::auto_log;
use crate::ast::expression::node::{ Identifier, InfixExpression, IntegerLiteral, PrefixExpression };
use crate::ast::expression::ExpressionType;
use crate::ast::statement::node::{ ExpressionStatement, LetStatement, ReturnStatement };
use crate::ast::statement::StatementType;
use crate::ast::Program;
use crate::{ lexer::Lexer, token::{ Precedence, Token } };
use core::option::Option;
use std::{ collections::HashMap, mem::Discriminant };

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
        Box::new(
            ExpressionType::Identifier(Identifier {
                token: self.cur_token.clone(),
            })
        )
    }

    #[auto_log]
    fn parse_integer_literal(&mut self) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        Box::new(ExpressionType::IntegerLiteral(IntegerLiteral { token }))
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

        Box::new(
            ExpressionType::Prefix(PrefixExpression {
                operator: token,
                right: Some(right),
            })
        )
    }

    pub fn peek_precedence(&self) -> Precedence {
        self.peek_token.to_precedence()
    }

    pub fn cur_precedence(&self) -> Precedence {
        self.cur_token.to_precedence()
    }

    #[auto_log]
    pub fn parse_infix_expression(&mut self, left: Box<ExpressionType>) -> Box<ExpressionType> {
        let token = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();

        Box::new(
            ExpressionType::Infix(InfixExpression {
                operator: token,
                left: Some(left),
                right: Some(self.parse_expression(precedence)),
            })
        )
    }

    pub fn register_prefix(&mut self, token_type: Token, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type.to_original_type(), function);
    }

    pub fn register_infix(&mut self, token_type: Token, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type.to_original_type(), function);
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, expected: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected,
            self.peek_token
        );
        self.errors.push(msg);
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
            self.peek_error(t.clone()); // Change to pass a reference to t
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<StatementType>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: None,
            value: None,
        };

        if !self.expect_peek(&Token::IDENT(String::new())) {
            return None;
        }

        stmt.name = Some(self.cur_token.to_string());

        if !self.expect_peek(&Token::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(StatementType::Let(stmt)))
    }

    fn parse_return_statement(&mut self) -> Option<Box<StatementType>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            value: None,
        };

        self.next_token();

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(StatementType::Return(stmt)))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<StatementType>> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: Some(self.parse_expression(Precedence::LOWEST)),
        };

        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(StatementType::Expression(stmt)))
    }

    fn parse_statement(&mut self) -> Option<Box<StatementType>> {
        match self.cur_token {
            Token::LET => {
                return self.parse_let_statement();
            }
            Token::RETURN => {
                return self.parse_return_statement();
            }
            _ => {
                return self.parse_expression_statement();
            } // _ => return None,
        }
    }

    #[auto_log]
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token != Token::EOF {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => {}
            }

            self.next_token();
        }

        program
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Box<ExpressionType> {
        let prefix = self.prefix_parse_fns.get(&self.cur_token.to_original_type()).cloned();

        match prefix {
            Some(prefix_fn) => {
                let mut left_exp = prefix_fn(self);
                while
                    !self.peek_token_is(Token::SEMICOLON) &&
                    self.cur_precedence() < self.peek_precedence()
                {
                    let infix = self.infix_parse_fns
                        .get(&self.peek_token.to_original_type())
                        .cloned();

                    match infix {
                        Some(infix_fn) => {
                            self.next_token();
                            left_exp = infix_fn(self, left_exp);
                        }
                        None => {
                            return left_exp;
                        }
                    }
                }
                left_exp
            }
            None => {
                let msg = format!("no prefix parse function for {:?}", self.cur_token);
                self.errors.push(msg);
                Box::new(ExpressionType::Identifier(Identifier { token: Token::EOF }))
            }
        }
    }
}
