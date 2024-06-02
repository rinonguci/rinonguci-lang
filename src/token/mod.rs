use lazy_static::lazy_static;
use std::collections::HashMap;
use std::mem::{ discriminant, Discriminant };

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST,
    EQUALS, // ==
    LESSGREATER, // > or <
    SUM, // +
    PRODUCT, // *
    PREFIX, // -X or !X
    CALL, // myFunction(X)
}

impl Precedence {
    pub fn to_int(&self) -> i32 {
        match self {
            Precedence::LOWEST => 1,
            Precedence::EQUALS => 2,
            Precedence::LESSGREATER => 3,
            Precedence::SUM => 4,
            Precedence::PRODUCT => 5,
            Precedence::PREFIX => 6,
            Precedence::CALL => 7,
        }
    }

    pub fn from_int(i: i32) -> Self {
        match i {
            1 => Precedence::LOWEST,
            2 => Precedence::EQUALS,
            3 => Precedence::LESSGREATER,
            4 => Precedence::SUM,
            5 => Precedence::PRODUCT,
            6 => Precedence::PREFIX,
            7 => Precedence::CALL,
            _ => Precedence::LOWEST,
        }
    }

    pub fn sum(self, i: i32) -> Self {
        Precedence::from_int(self.to_int() + i)
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
#[allow(non_camel_case_types)]
pub enum Token {
    EOF,
    ILLEGAL(char),

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG, // !
    ASTERISK, // *
    SLASH, // /

    GT, // >
    LT, // <
    EQ, // ==
    NOT_EQ, // !=

    //Delimeters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::ILLEGAL(c) => c.to_string(),
            Token::IDENT(s) => s.to_string(),
            Token::INT(i) => i.to_string(),
            Token::EOF => String::from("EOF"),
            Token::ASSIGN => String::from("="),
            Token::PLUS => String::from("+"),
            Token::MINUS => String::from("-"),
            Token::BANG => String::from("!"),
            Token::ASTERISK => String::from("*"),
            Token::SLASH => String::from("/"),
            Token::GT => String::from(">"),
            Token::LT => String::from("<"),
            Token::EQ => String::from("=="),
            Token::NOT_EQ => String::from("!="),
            Token::COMMA => String::from(","),
            Token::SEMICOLON => String::from(";"),
            Token::LPAREN => String::from("("),
            Token::RPAREN => String::from(")"),
            Token::LBRACE => String::from("{"),
            Token::RBRACE => String::from("}"),
            Token::FUNCTION => String::from("fn"),
            Token::LET => String::from("let"),
            Token::TRUE => String::from("true"),
            Token::FALSE => String::from("false"),
            Token::IF => String::from("if"),
            Token::ELSE => String::from("else"),
            Token::RETURN => String::from("return"),
        }
    }

    pub fn is(&self, t: Token) -> bool {
        discriminant(self) == discriminant(&t)
    }

    pub fn to_original_type(&self) -> Discriminant<Self> {
        return discriminant(&self);
    }

    pub fn to_precedence(&self) -> Precedence {
        match self {
            Token::EQ => Precedence::EQUALS,
            Token::NOT_EQ => Precedence::EQUALS,
            Token::LT => Precedence::LESSGREATER,
            Token::GT => Precedence::LESSGREATER,
            Token::PLUS => Precedence::SUM,
            Token::MINUS => Precedence::SUM,
            Token::SLASH => Precedence::PRODUCT,
            Token::ASTERISK => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("fn", Token::FUNCTION);
        m.insert("let", Token::LET);
        m.insert("true", Token::TRUE);
        m.insert("false", Token::FALSE);
        m.insert("if", Token::IF);
        m.insert("else", Token::ELSE);
        m.insert("return", Token::RETURN);
        m
    };
}
