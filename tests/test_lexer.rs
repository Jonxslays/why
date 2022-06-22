#[allow(dead_code)]
const SOURCE: &'static str = {
    r#"

// Iterate each item in the list
// equivalent to for each loops
my_list@->item {
    // % is a qualifier for the stdlib (not required)
    // but it can allow you to shadow builtins
    %print(item);
}
// Iterate from 1 to 20 exclusive
1->20@->number {
    print(number);
}
// Iterate from 1 to 20 inclusive
1=>20@->number {
    print(number);
}
// Iterate the mapping keys and access each element
my_dict@->key {
    print(dict@key);
    // or
    print(dict[key]);
    // or
    print(dict.get(key))
    // Keep in mind this last one can return NULL
}
// Iterate the mapping keys and values simultaneously
my_dict@=>(key, value) {
    print(key);
    print(value);
}
int number = 0;
// A while true loop
@->true! {
    (number > 100)? {
        @!; // Break out of the loop early
    } !-> {
        print("Number: {++number}");
    }
}
"#
};

#[cfg(test)]
mod test_vars {
    // use crate::SOURCE;
    use why_rs::Lexer;
    use why_rs::Loc;
    use why_rs::Token;
    use why_rs::TokenType;
    use why_rs::WhyExc;

    #[test]
    #[rustfmt::skip]
    fn test_lexer_lexes_int_definition() -> Result<(), WhyExc> {
        let src = "int my_num = 69;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Ident, value: "my_num".to_string(), loc: Loc { line: 1, col: 5 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "69".to_string(), loc: Loc { line: 1, col: 14 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 16 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 17 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexer_lexes_float_definition() -> Result<(), WhyExc> {
        let src = "float my_float = 69.420;";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "float".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::Ident, value: "my_float".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 16 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "69.420".to_string(), loc: Loc { line: 1, col: 18 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 24 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 25 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_lexer_fails_on_invalid_float_definition() {
        let src = "float my_float = 69.420.69;";
        let mut lexer = Lexer::new(src).unwrap();
        let _received_tokens = lexer.lex().unwrap();
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexer_lexes_array_definition() -> Result<(), WhyExc> {
        let src = "array@int my_list = [1, 2, 3, 4, 5, 6, 7];";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "array".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 6 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 7 }, addtl: None },
            Token { typ: TokenType::Ident, value: "my_list".to_string(), loc: Loc { line: 1, col: 11 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 19 }, addtl: None },
            Token { typ: TokenType::LBracket, value: "[".to_string(), loc: Loc { line: 1, col: 21 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "1".to_string(), loc: Loc { line: 1, col: 22 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 23 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "2".to_string(), loc: Loc { line: 1, col: 25 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 26 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "3".to_string(), loc: Loc { line: 1, col: 28 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 29 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "4".to_string(), loc: Loc { line: 1, col: 31 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 32 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "5".to_string(), loc: Loc { line: 1, col: 34 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 35 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "6".to_string(), loc: Loc { line: 1, col: 37 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 38 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "7".to_string(), loc: Loc { line: 1, col: 40 }, addtl: None },
            Token { typ: TokenType::RBracket, value: "]".to_string(), loc: Loc { line: 1, col: 41 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 42 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 43 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_lexer_lexes_mapping_definition() -> Result<(), WhyExc> {
        let src = "mapping@int->int my_dict = &{ 1->2, 3->4 };";
        let mut lexer = Lexer::new(src)?;
        let received_tokens = lexer.lex()?;

        let expected_tokens: Vec<Token> = vec![
            Token { typ: TokenType::Ident, value: "mapping".to_string(), loc: Loc::default(), addtl: None },
            Token { typ: TokenType::At, value: "@".to_string(), loc: Loc { line: 1, col: 8 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 9 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 12 }, addtl: None },
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc { line: 1, col: 14 }, addtl: None },
            Token { typ: TokenType::Ident, value: "my_dict".to_string(), loc: Loc { line: 1, col: 18 }, addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc { line: 1, col: 26 }, addtl: None },
            Token { typ: TokenType::And, value: "&".to_string(), loc: Loc { line: 1, col: 28 }, addtl: None },
            Token { typ: TokenType::LBrace, value: "{".to_string(), loc: Loc { line: 1, col: 29 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "1".to_string(), loc: Loc { line: 1, col: 31 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 32 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "2".to_string(), loc: Loc { line: 1, col: 34 }, addtl: None },
            Token { typ: TokenType::Comma, value: ",".to_string(), loc: Loc { line: 1, col: 35 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "3".to_string(), loc: Loc { line: 1, col: 37 }, addtl: None },
            Token { typ: TokenType::SmallRArrow, value: "->".to_string(), loc: Loc { line: 1, col: 38 }, addtl: None },
            Token { typ: TokenType::NumLiteral, value: "4".to_string(), loc: Loc { line: 1, col: 40 }, addtl: None },
            Token { typ: TokenType::RBrace, value: "}".to_string(), loc: Loc { line: 1, col: 42 }, addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc { line: 1, col: 43 }, addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc { line: 1, col: 44 }, addtl: None },
        ];

        assert_eq!(expected_tokens, received_tokens);
        Ok(())
    }
}
