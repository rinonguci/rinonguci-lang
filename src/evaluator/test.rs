#[cfg(test)]
mod tests {
    use crate::{
        evaluator::eval,
        lexer::Lexer,
        object::{Integer, Null, Object},
        parser::Parser,
    };

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
        for tt in tests {
            let evaluated = test_eval(tt.0);
            test_integer_object(evaluated, tt.1);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];
        for tt in tests {
            let evaluated = test_eval(tt.0);
            test_boolean_object(evaluated, tt.1);
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];
        for tt in tests {
            let evaluated = test_eval(tt.0);
            test_boolean_object(evaluated, tt.1);
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", Object::Integer(Integer { value: 10 })),
            ("if (false) { 10 }", Object::Null(Null)),
            ("if (1) { 10 }", Object::Integer(Integer { value: 10 })),
            ("if (1 < 2) { 10 }", Object::Integer(Integer { value: 10 })),
            ("if (1 > 2) { 10 }", Object::Null(Null)),
            (
                "if (1 > 2) { 10 } else { 20 }",
                Object::Integer(Integer { value: 20 }),
            ),
            (
                "if (1 < 2) { 10 } else { 20 }",
                Object::Integer(Integer { value: 10 }),
            ),
        ];

        for tt in tests {
            let evaluated = test_eval(tt.0);
            if let Object::Integer(integer) = tt.1 {
                test_integer_object(evaluated, integer.value);
            } else {
                test_null_object(evaluated);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                "
                if (10 > 1){
                    if (10 > 1){
                        return 10;
                    }

                    return 1
                }
            ",
                10,
            ),
        ];
        for tt in tests {
            let evaluated = test_eval(tt.0)
                .into_return()
                .expect("not a return statement");
            test_integer_object(evaluated.value.as_ref().clone(), tt.1);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
        ];
        for tt in tests {
            let evaluated = test_eval(tt.0);
            let err_obj = evaluated.into_error().expect("not an error object");
            assert_eq!(
                err_obj.message, tt.1,
                "expected={}, got={}",
                tt.1, err_obj.message
            );
        }
    }

    fn test_eval(input: &str) -> Object {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        eval(Box::new(program.to_node()))
    }

    fn test_integer_object(obj: Object, expected: i64) {
        let result = obj.as_integer().unwrap();
        assert_eq!(
            result.value, expected,
            "expected={}, got={}",
            expected, result.value
        );
    }

    fn test_boolean_object(obj: Object, expected: bool) {
        let result = obj.as_boolean().unwrap();
        assert_eq!(result.value, expected);
    }

    fn test_null_object(obj: Object) {
        assert_eq!(obj, Object::Null(Null));
    }
}
