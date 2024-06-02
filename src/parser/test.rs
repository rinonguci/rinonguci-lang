#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            expression::{node::Identifier, ExpressionType},
            statement::{node::LetStatement, StatementType},
            Node, Program,
        },
        lexer::Lexer,
        parser::Parser,
        token::Token,
    };

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

        assert_eq!(
            program.statements.len(),
            3,
            "program.Statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let tests = vec![("x", Some(1)), ("y", Some(10)), ("foobar", Some(838383))];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            let stmt = stmt.as_let().unwrap();

            assert_eq!(
                stmt.name, tt.0,
                "stmt.Name.Value not '{}'. got={}",
                tt.0, stmt.name,
            );
        }
    }

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(StatementType::Let(LetStatement {
                token: Token::LET,
                name: "myVar".to_string(),
                value: Some(Box::new(ExpressionType::Identifier(Identifier {
                    token: Token::IDENT("anotherVar".to_string()),
                }))),
            }))],
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

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements"
        );

        let stmt = program.statements[0]
            .as_expression()
            .unwrap()
            .expression
            .as_ref()
            .token_literal();

        assert_eq!(stmt, "foobar", "ident.Value not 'foobar'");
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements"
        );

        let stmt = program.statements[0].as_expression().unwrap();

        assert_eq!(
            stmt.expression.as_ref().token_literal(),
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

            assert_eq!(
                program.statements.len(),
                1,
                "program.Statements does not contain 1 statement"
            );

            let stmt = &program.statements[0].as_expression().unwrap();
            let exp = stmt.expression.as_ref().as_prefix().unwrap();

            assert_eq!(
                exp.operator.to_string(),
                operator,
                "exp.Operator is not '{}'. got={}",
                operator,
                exp.operator.to_string()
            );

            assert_eq!(
                exp.right.as_ref().token_literal(),
                integer_value.to_string(),
                "exp.Right.Value is not '{}'. got={}",
                integer_value,
                exp.right.as_ref().token_literal()
            );
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = vec![
            ("5 + 6", 5, "+", 6),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left_value, operator, right_value) in infix_tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();

            assert_eq!(
                program.statements.len(),
                1,
                "program.Statements does not contain 1 statement"
            );

            let stmt = program.statements[0].as_expression().unwrap();
            let exp = stmt.expression.as_ref().as_infix().unwrap();

            assert_eq!(
                exp.operator.to_string(),
                operator,
                "exp.Operator is not '{}'. got={}",
                operator,
                exp.operator.to_string()
            );

            assert_eq!(
                exp.left.as_ref().token_literal(),
                left_value.to_string(),
                "exp.Left.Value is not '{}'. got={}",
                left_value,
                exp.left.as_ref().token_literal()
            );

            assert_eq!(
                exp.right.as_ref().token_literal(),
                right_value.to_string(),
                "exp.Right.Value is not '{}'. got={}",
                right_value,
                exp.right.as_ref().token_literal()
            );
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b * c", "(a + (b * c))"),
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

            let actual = program.string();

            assert_eq!(actual, expected, "expected={}, got={}", expected, actual);
        }
    }
}
