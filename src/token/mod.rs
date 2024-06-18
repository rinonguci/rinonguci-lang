use enum_as_inner::EnumAsInner;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Hash, Eq, EnumAsInner)]
pub enum Token {
    EOF,
    ILLEGAL(char),

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    GT,     // >
    LT,     // <
    EQ,     // ==
    NOT_EQ, // !=

    //Delimeters
    COMMA,     // ,
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }

    // keywords
    FUNCTION,
    LET,
    // REASSIGN,
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
            Token::EOF => "EOF".into(),
            Token::ASSIGN => "=".into(),
            Token::PLUS => "+".into(),
            Token::MINUS => "-".into(),
            Token::BANG => "!".into(),
            Token::ASTERISK => "*".into(),
            Token::SLASH => "/".into(),
            Token::GT => ">".into(),
            Token::LT => "<".into(),
            Token::EQ => "==".into(),
            Token::NOT_EQ => "!=".into(),
            Token::COMMA => ",".into(),
            Token::SEMICOLON => ";".into(),
            Token::LPAREN => "(".into(),
            Token::RPAREN => ")".into(),
            Token::LBRACE => "{".into(),
            Token::RBRACE => "}".into(),
            Token::FUNCTION => "fn".into(),
            Token::LET => "let".into(),
            // Token::REASSIGN => "=".into(),
            Token::TRUE => "true".into(),
            Token::FALSE => "false".into(),
            Token::IF => "if".into(),
            Token::ELSE => "else".into(),
            Token::RETURN => "return".into(),
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
            Token::LPAREN => Precedence::CALL,
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
