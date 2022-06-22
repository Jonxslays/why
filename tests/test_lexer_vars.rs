const SOURCE: &'static str = {
r#"int one = 1;
bool my_bool = false;
"#
};

#[cfg(test)]
mod test_vars {
    use crate::SOURCE;
    use why_rs::Loc;
    use why_rs::Lexer;
    use why_rs::Token;
    use why_rs::TokenType;

    #[test]
    fn test_lexer_lexes_vars() {
        let expected_tokens = vec![
            Token { typ: TokenType::Ident, value: "int".to_string(), loc: Loc::at(1, 1), addtl: None },
            Token { typ: TokenType::Ident, value: "one".to_string(), loc: Loc::at(1, 5), addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc::at(1, 9), addtl: None },
            Token { typ: TokenType::Ident, value: "1".to_string(), loc: Loc::at(1, 11), addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc::at(1, 12), addtl: None },
            Token { typ: TokenType::Ident, value: "bool".to_string(), loc: Loc::at(2, 1), addtl: None },
            Token { typ: TokenType::Ident, value: "my_bool".to_string(), loc: Loc::at(2, 6), addtl: None },
            Token { typ: TokenType::Eq, value: "=".to_string(), loc: Loc::at(2, 14), addtl: None },
            Token { typ: TokenType::Ident, value: "false".to_string(), loc: Loc::at(2, 16), addtl: None },
            Token { typ: TokenType::Semi, value: ";".to_string(), loc: Loc::at(2, 21), addtl: None },
            Token { typ: TokenType::Eof, value: "".to_string(), loc: Loc::at(3, 1), addtl: None },
        ];

        let mut lexer = Lexer::new(SOURCE.to_string());
        let received_tokens = lexer.lex();

        assert_eq!(expected_tokens, received_tokens);
    }
}
