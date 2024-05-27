use crate::ast::ast::InfixExpression;
#[allow(unused_imports)]
use crate::{
    ast::ast::{
        Expression, ExpressionStatement, ExpressionValue, Identifier, IntegerLiteral, LetStatement,
        Node, PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::lexer::Lexer,
    token::token::{Precedence, Token},
};
use core::option::Option;
use std::{collections::HashMap, mem::Discriminant};

type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>; // Change the function signature to accept a mutable reference to self.
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>;

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

    pub fn parse_identifier(&mut self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        Box::new(IntegerLiteral { token })
    }

    pub fn no_prefix_parse_fn_error(&mut self, t: Token) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(msg);
    }

    pub fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        println!("{:#?}", self);
        let token = self.cur_token.clone();
        let operator = self.cur_token.to_string();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);
        Box::new(PrefixExpression {
            operator,
            token,
            right: Some(right),
        })
    }

    pub fn peek_precedence(&self) -> Precedence {
        self.peek_token.to_precedence()
    }

    pub fn cur_precedence(&self) -> Precedence {
        self.cur_token.to_precedence()
    }

    pub fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.to_string();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Box::new(InfixExpression {
            operator,
            token,
            left: Some(left),
            right: Some(right),
        })
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

    fn peek_error(&mut self, expected: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected, self.peek_token
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

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
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

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            value: None,
        };

        self.next_token();

        while !self.cur_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: Some(self.parse_expression(Precedence::LOWEST)),
        };

        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            Token::LET => {
                return self.parse_let_statement();
            }
            Token::RETURN => {
                return self.parse_return_statement();
            }
            _ => return self.parse_expression_statement(),
            // _ => return None,
        }
    }

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

    fn parse_expression(&mut self, _precedence: Precedence) -> Box<dyn Expression> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.cur_token.to_original_type())
            .cloned();

        match prefix {
            Some(prefix_fn) => {
                let mut left_exp = prefix_fn(self);
                while !self.peek_token_is(Token::SEMICOLON)
                    && self.cur_precedence() < self.peek_precedence()
                {
                    let infix = self
                        .infix_parse_fns
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
                Box::new(ExpressionValue {
                    token: self.cur_token.clone(),
                })
            }
        }
    }
}

#[allow(dead_code)]
fn check_parser_errors(p: &Parser) -> Result<(), String> {
    let errors = p.errors();

    if errors.is_empty() {
        return Ok(());
    }

    let mut error_msg = format!("\nParser has {} errors\n", errors.len());
    for msg in errors {
        error_msg.push_str(&format!("Parser error: {}\n", msg));
    }

    println!("============ERROR============");
    println!("{}", error_msg);
    println!("============ERROR============");
    Err(error_msg)
}

#[test]
fn test_let_statements() {
    let input = "
    let x = 1;
    let y = 10;
    let foobar = 838383;
    ";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    match check_parser_errors(&p) {
        Ok(_) => {}
        Err(msg) => panic!("{:?}", msg),
    }

    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let tests = vec![("x", Some(1)), ("y", Some(10)), ("foobar", Some(838383))];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        assert_eq!(
            test_let_statement(stmt, tt.0, &tt.1),
            true,
            "test_let_statement failed"
        );
    }
}

#[allow(dead_code)]
fn test_let_statement(stmt: &Box<dyn Statement>, name: &str, _value: &Option<i64>) -> bool {
    let let_stmt = stmt
        .as_ref()
        .as_any()
        .downcast_ref::<LetStatement>()
        .unwrap();

    if let_stmt.name.as_ref().unwrap() != name {
        return false;
    }

    true
}

#[test]
fn test_string() {
    let program = Program {
        statements: vec![Box::new(LetStatement {
            token: Token::LET,
            name: Some("myVar".to_string()),
            value: Some(Box::new(ExpressionValue {
                token: Token::IDENT("anotherVar".to_string()),
            })),
        })],
    };

    let expected_output = "let myVar = anotherVar;";
    assert_eq!(program.string(), expected_output, "program.String() wrong");
}

#[test]
fn test_return_statements() {
    let input = "
    return 5;
    return 10;
    return 993322;
    ";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    match check_parser_errors(&p) {
        Ok(_) => {}
        Err(msg) => panic!("{:?}", msg),
    }

    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for stmt in program.statements {
        assert_eq!(
            stmt.token_literal(),
            "return".to_string(),
            "stmt.TokenLiteral not 'return'. got={}",
            stmt.token_literal()
        );
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    match check_parser_errors(&mut p) {
        Ok(_) => {}
        Err(msg) => panic!("{:?}", msg),
    }
    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements"
    );

    println!("{:#?}", program);

    let stmt = &program.statements[0];
    let stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();

    assert_eq!(
        stmt.expression.as_ref().unwrap().token_literal(),
        "foobar",
        "ident.Value not 'foobar'"
    );
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    match check_parser_errors(&mut p) {
        Ok(_) => {}
        Err(msg) => panic!("{:?}", msg),
    }
    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements"
    );

    let stmt = &program.statements[0];
    let stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();

    assert_eq!(
        stmt.expression.as_ref().unwrap().token_literal(),
        "5",
        "ident.Value not '5'"
    );
}

#[test]
fn test_parsing_prefix_expressions() {
    let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

    for (input, operator, integer_value) in prefix_tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        match check_parser_errors(&mut p) {
            Ok(_) => {}
            Err(msg) => panic!("{:?}", msg),
        }

        assert_eq!(
            program.statements.len(),
            1,
            "program.Statements does not contain 1 statement"
        );

        let stmt = &program.statements[0];
        let stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();
        let exp = stmt.expression.as_ref().unwrap();
        let exp = exp.as_any().downcast_ref::<PrefixExpression>().unwrap();

        assert_eq!(
            exp.operator, operator,
            "exp.Operator is not '{}'. got={}",
            operator, exp.operator
        );

        assert_eq!(
            exp.right.as_ref().unwrap().token_literal(),
            integer_value.to_string(),
            "exp.Right.Value is not '{}'. got={}",
            integer_value,
            exp.right.as_ref().unwrap().token_literal()
        );
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let infix_tests = vec![
        ("5 + 5 + 9;", 5, "+", 5),
        // ("5 - 5;", 5, "-", 5),
        // ("5 * 5;", 5, "*", 5),
        // ("5 / 5;", 5, "/", 5),
        // ("5 > 5;", 5, ">", 5),
        // ("5 < 5;", 5, "<", 5),
        // ("5 == 5;", 5, "==", 5),
        // ("5 != 5;", 5, "!=", 5),
    ];

    for (input, left_value, operator, right_value) in infix_tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        match check_parser_errors(&mut p) {
            Ok(_) => {}
            Err(msg) => panic!("{:?}", msg),
        }

        assert_eq!(
            program.statements.len(),
            1,
            "program.Statements does not contain 1 statement"
        );

        println!("{:#?}", program);

        let stmt = &program.statements[0];
        let stmt = stmt.as_any().downcast_ref::<ExpressionStatement>().unwrap();
        let exp = stmt.expression.as_ref().unwrap();
        let exp = exp.as_any().downcast_ref::<InfixExpression>().unwrap();

        assert_eq!(
            exp.operator, operator,
            "exp.Operator is not '{}'. got={}",
            operator, exp.operator
        );

        assert_eq!(
            exp.left.as_ref().unwrap().token_literal(),
            left_value.to_string(),
            "exp.Left.Value is not '{}'. got={}",
            left_value,
            exp.left.as_ref().unwrap().token_literal()
        );

        assert_eq!(
            exp.right.as_ref().unwrap().token_literal(),
            right_value.to_string(),
            "exp.Right.Value is not '{}'. got={}",
            right_value,
            exp.right.as_ref().unwrap().token_literal()
        );
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        match check_parser_errors(&mut p) {
            Ok(_) => {}
            Err(msg) => panic!("{:?}", msg),
        }
        let actual = program.string();

        assert_eq!(actual, expected, "expected={}, got={}", expected, actual);
    }
}
