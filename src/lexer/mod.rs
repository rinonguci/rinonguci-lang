pub mod test;

use crate::token;
use token::{Token, KEYWORDS};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination, using Option for handling end of input
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '\0' => Token::EOF,
            '=' if self.peek_char() == '=' => {
                self.read_char();
                Token::EQ
            }
            '!' if self.peek_char() == '=' => {
                self.read_char();
                Token::NOT_EQ
            }
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '!' => Token::BANG,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '>' => Token::GT,
            '<' => Token::LT,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            c if c.is_digit(10) => return Token::INT(self.read_number().parse().unwrap()),
            c if is_letter(c) => {
                let str = self.read_identifier();
                return match KEYWORDS.contains_key(str.as_str()) {
                    true => KEYWORDS.get(str.as_str()).unwrap().clone(),
                    false => Token::IDENT(str),
                };
            }
            c => Token::ILLEGAL(c),
        };

        self.read_char();
        token
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
