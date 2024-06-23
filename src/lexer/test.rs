#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test() {
        let input = r#"
          let five = 5;
          let ten = 10;
          let add = fn(x, y) {
            x + y;
          };
          let result = add(five, ten);
          !-/*6;
          7 < 10 > 8;

          if (9 < 11) {
            return true;
          } else {
            return false;
          }

          13 == 13;
          14 != 5;
          "foobar"
          "foo bar"
        "#;

        let tokens = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(6),
            Token::SEMICOLON,
            Token::INT(7),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(8),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(9),
            Token::LT,
            Token::INT(11),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(13),
            Token::EQ,
            Token::INT(13),
            Token::SEMICOLON,
            Token::INT(14),
            Token::NOT_EQ,
            Token::INT(5),
            Token::SEMICOLON,
            Token::STRING("foobar".into()),
            Token::STRING("foo bar".into()),
            Token::EOF,
        ];

        let mut l = Lexer::new(input.to_string());

        for token in tokens {
            assert_eq!(l.next_token(), token);
        }
    }
}
